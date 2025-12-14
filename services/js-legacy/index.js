const fs = require('fs');
const path = require('path');
const { Pool } = require('pg');
const copyFrom = require('pg-copy-streams').from;
const exceljs = require('exceljs');
const csv = require('fast-csv');

// --- Configuration ---
const
    OUT_DIR = process.env.CSV_OUT_DIR || '/data/csv',
    PG_HOST = process.env.PGHOST || 'db',
    PG_PORT = process.env.PGPORT || 5432,
    PG_USER = process.env.PGUSER || 'monouser',
    PG_DB = process.env.PGDATABASE || 'monolith',
    PG_PASSWORD = process.env.PGPASSWORD || 'monopass';

// --- Utility Functions ---
const log = (message) => console.log(`[GenService] INFO: ${new Date().toISOString()} ${message}`);
const error = (message, err) => console.error(`[GenService] ERROR: ${new Date().toISOString()} ${message}`, err);

const randFloat = (min, max) => min + Math.random() * (max - min);
const randInt = (min, max) => Math.floor(Math.random() * (max - min + 1)) + min;

// --- Core Logic ---

/**
 * Generates a single telemetry data record.
 * @param {string} sourceFilename - The name of the file this data will be associated with.
 * @returns {object} A telemetry data object.
 */
function generateTelemetryData(sourceFilename) {
    return {
        recordedAt: new Date(),
        voltage: randFloat(3.2, 12.6),
        temp: randFloat(-50.0, 80.0),
        isValid: randInt(0, 1) === 1,
        sourceFile: sourceFilename,
    };
}

/**
 * Saves telemetry data to a CSV file.
 * @param {string} filepath - The full path to the output CSV file.
 * @param {object} data - The telemetry data object.
 * @returns {Promise<void>}
 */
function saveCsv(filepath, data) {
    return new Promise((resolve, reject) => {
        const ws = fs.createWriteStream(filepath);
        const rows = [
            ['recorded_at', 'voltage', 'temp', 'is_valid', 'source_file'],
            [
                data.recordedAt.toISOString(),
                data.voltage.toFixed(2),
                data.temp.toFixed(2),
                data.isValid ? 'true' : 'false',
                data.sourceFile
            ]
        ];

        csv.write(rows, { headers: true })
            .pipe(ws)
            .on('finish', () => {
                log(`CSV generated: ${filepath}`);
                resolve();
            })
            .on('error', (err) => reject(err));
    });
}

/**
 * Saves telemetry data to an XLSX file.
 * @param {string} filepath - The full path to the output XLSX file.
 * @param {object} data - The telemetry data object.
 * @returns {Promise<void>}
 */
async function saveXlsx(filepath, data) {
    const workbook = new exceljs.Workbook();
    const sheet = workbook.addWorksheet('Telemetry');

    sheet.columns = [
        { header: 'Time (Timestamp)', key: 'recordedAt', width: 25 },
        { header: 'Voltage (Num)', key: 'voltage', width: 15 },
        { header: 'Temp (Num)', key: 'temp', width: 15 },
        { header: 'Valid (Bool)', key: 'isValid', width: 15 },
        { header: 'Source (Text)', key: 'sourceFile', width: 40 },
    ];

    sheet.getRow(2).values = {
        ...data,
        recordedAt: data.recordedAt, 
    };
    
    // Apply date format style
    sheet.getCell('A2').numFmt = 'yyyy-mm-dd hh:mm:ss';

    await workbook.xlsx.writeFile(filepath);
    log(`XLSX generated: ${filepath}`);
}


/**
 * Imports data from a CSV file into the PostgreSQL database.
 * @param {string} csvFilepath - The full path to the CSV file.
 */
async function importToDb(csvFilepath) {
    log(`Starting DB import for ${csvFilepath}`);
    const pool = new Pool({
        host: PG_HOST,
        port: PG_PORT,
        user: PG_USER,
        database: PG_DB,
        password: PG_PASSWORD,
    });

    const client = await pool.connect();
    log('Successfully connected to the database.');

    return new Promise((resolve, reject) => {
        const stream = client.query(
            copyFrom(`COPY telemetry_legacy(recorded_at, voltage, temp, is_valid, source_file) FROM STDIN WITH (FORMAT csv, HEADER true)`)
        );
        const fileStream = fs.createReadStream(csvFilepath);

        fileStream.on('error', (err) => {
            error('Error reading CSV file for DB import.', err);
            client.release();
            pool.end();
            reject(err);
        });

        stream.on('error', (err) => {
            error('Error during database COPY operation.', err);
            client.release();
            pool.end();
            reject(err);
        });

        stream.on('finish', () => {
            log('Database import finished successfully.');
            client.release();
            pool.end();
            resolve();
        });

        fileStream.pipe(stream);
    });
}


/**
 * Main function to run the entire generation and import process.
 */
async function main() {
    log('Starting generation cycle.');
    try {
        if (!fs.existsSync(OUT_DIR)) {
            fs.mkdirSync(OUT_DIR, { recursive: true });
            log(`Created output directory: ${OUT_DIR}`);
        }

        const ts = new Date().toISOString().replace(/[:.]/g, '-');
        const baseName = `telemetry_${ts}`;
        const csvFilename = `${baseName}.csv`;
        const xlsxFilename = `${baseName}.xlsx`;
        const csvFilepath = path.join(OUT_DIR, csvFilename);
        const xlsxFilepath = path.join(OUT_DIR, xlsxFilename);

        const data = generateTelemetryData(csvFilename);

        await saveCsv(csvFilepath, data);
        await saveXlsx(xlsxFilepath, data);
        await importToDb(csvFilepath);

        log('Generation cycle finished successfully.');
    } catch (err) {
        error('An error occurred during the generation cycle.', err);
        process.exit(1);
    }
}

main();
