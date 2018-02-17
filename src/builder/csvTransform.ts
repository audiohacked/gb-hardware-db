import * as fs from 'fs';
import * as csvStringify from 'csv-stringify';
import {
  AgbSubmission, AgsSubmission, CgbSubmission, DmgSubmission, GbsSubmission, MgbSubmission, MglSubmission,
  OxySubmission,
  Sgb2Submission, SgbSubmission, ConsoleSubmission
} from '../crawler';
import {
  AgbMetadata, AgsMetadata, CgbMetadata, Chip, DmgMetadata, GbsMetadata, MgbMetadata, MglMetadata, OxyMetadata,
  Sgb2Metadata,
  SgbMetadata
} from '../metadata';
import * as format from '../site/format';

export interface CsvColumn<T> {
  name: string,
  get: (value: T) => any,
}

export function generateCsv<T>(columns: CsvColumn<T>[], rows: T[], path: string): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    const stringifier = csvStringify({
      columns: columns.map(({name}) => name),
      header: true,
    });
    const stream = fs.createWriteStream(path);
    stringifier.pipe(stream, {end: true})
      .on('error', reject)
      .on('finish', resolve);

    for (const row of rows) {
      stringifier.write(columns.map(({get}) => get(row)), 'UTF-8');
    }
    stringifier.end();
  })
}

const SUBMISSION_COLUMNS: CsvColumn<ConsoleSubmission>[] = [
  field('', 'type'),
  field('', 'title'),
  field('', 'slug'),
  generate('', 'url', s => `https://gbhwdb.gekkio.fi/consoles/${s.type}/${s.slug}.html`),
  field('', 'contributor'),
];

export const DMG_CSV_COLUMNS: CsvColumn<DmgSubmission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: DmgSubmission) => s.metadata, [
    field('', 'color'),
    field('', 'screws'),
    generate('', 'calendar_short', format.short.calendar),
    generate('', 'calendar', format.calendar),
    field('', 'year'),
    field('', 'month'),
    ...lift((m: DmgMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'extra_label'),
      field('mainboard', 'stamp'),
      field('mainboard', 'circled_letters'),
    ]),
    ...lift((m: DmgMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: DmgMetadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: DmgMetadata) => m.mainboard.video_ram, chipColumns('video_ram')),
    ...lift((m: DmgMetadata) => m.mainboard.amplifier, chipColumns('amplifier')),
    ...lift((m: DmgMetadata) => m.mainboard.crystal, chipColumns('crystal')),
    ...lift((m: DmgMetadata) => m.lcd_board, [
      field('lcd_board', 'type'),
      field('lcd_board', 'circled_letters'),
      field('lcd_board', 'stamp'),
      generate('lcd_board', 'calendar_short', format.short.calendar),
      generate('lcd_board', 'calendar', format.calendar),
      field('lcd_board', 'year'),
      field('lcd_board', 'month'),
    ]),
    ...lift((m: DmgMetadata) => m.lcd_board && m.lcd_board.column_driver, chipColumns('column_driver')),
    ...lift((m: DmgMetadata) => m.lcd_board && m.lcd_board.row_driver, chipColumns('row_driver')),
    ...lift((m: DmgMetadata) => m.lcd_board && m.lcd_board.regulator, chipColumns('regulator')),
    ...lift((m: DmgMetadata) => m.power_board, [
      field('power_board', 'type'),
      field('power_board', 'label'),
      generate('power_board', 'calendar_short', format.short.calendar),
      generate('power_board', 'calendar', format.calendar),
      field('power_board', 'year'),
      field('power_board', 'month'),
    ]),
    ...lift((m: DmgMetadata) => m.jack_board, [
      field('jack_board', 'type'),
      field('jack_board', 'extra_label'),
    ])
  ]),
];

export const SGB_CSV_COLUMNS: CsvColumn<SgbSubmission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: SgbSubmission) => s.metadata, [
    field('', 'stamp'),
    ...lift((m: SgbMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'circled_letters'),
      field('mainboard', 'letter_at_top_right'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
    ]),
    ...lift((m: SgbMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: SgbMetadata) => m.mainboard.icd2, chipColumns('icd2')),
    ...lift((m: SgbMetadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: SgbMetadata) => m.mainboard.video_ram, chipColumns('video_ram')),
    ...lift((m: SgbMetadata) => m.mainboard.rom, chipColumns('rom')),
    ...lift((m: SgbMetadata) => m.mainboard.cic, chipColumns('cic')),
  ]),
];

export const MGB_CSV_COLUMNS: CsvColumn<MgbSubmission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: MgbSubmission) => s.metadata, [
    field('', 'color'),
    generate('', 'calendar_short', format.short.calendar),
    generate('', 'calendar', format.calendar),
    field('', 'year'),
    field('', 'month'),
    ...lift((m: MgbMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'number_pair'),
      field('mainboard', 'stamp'),
      field('mainboard', 'circled_letters'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
      // TODO: date_range
    ]),
    ...lift((m: MgbMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: MgbMetadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: MgbMetadata) => m.mainboard.amplifier, chipColumns('amplifier')),
    ...lift((m: MgbMetadata) => m.mainboard.regulator, chipColumns('regulator')),
    ...lift((m: MgbMetadata) => m.mainboard.crystal, chipColumns('crystal')),
    ...lift((m: MgbMetadata) => m.lcd && m.lcd.column_driver, chipColumns('column_driver')),
    ...lift((m: MgbMetadata) => m.lcd && m.lcd.row_driver, chipColumns('row_driver')),
  ]),
];

export const MGL_CSV_COLUMNS: CsvColumn<MglSubmission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: MglSubmission) => s.metadata, [
    field('', 'color'),
    generate('', 'calendar_short', format.short.calendar),
    generate('', 'calendar', format.calendar),
    field('', 'year'),
    field('', 'week'),
    ...lift((m: MglMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'number_pair'),
      field('mainboard', 'stamp'),
      field('mainboard', 'circled_letters'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
      // TODO: date_range
    ]),
    ...lift((m: MglMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: MglMetadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: MglMetadata) => m.mainboard.amplifier, chipColumns('amplifier')),
    ...lift((m: MglMetadata) => m.mainboard.regulator, chipColumns('regulator')),
    ...lift((m: MglMetadata) => m.mainboard.crystal, chipColumns('crystal')),
    ...lift((m: MglMetadata) => m.mainboard.t1, chipColumns('t1')),
  ]),
];

export const SGB2_CSV_COLUMNS: CsvColumn<Sgb2Submission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: Sgb2Submission) => s.metadata, [
    field('', 'stamp'),
    ...lift((m: Sgb2Metadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'circled_letters'),
      field('mainboard', 'letter_at_top_right'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
    ]),
    ...lift((m: Sgb2Metadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: Sgb2Metadata) => m.mainboard.icd2, chipColumns('icd2')),
    ...lift((m: Sgb2Metadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: Sgb2Metadata) => m.mainboard.rom, chipColumns('rom')),
    ...lift((m: Sgb2Metadata) => m.mainboard.cic, chipColumns('cic')),
    ...lift((m: Sgb2Metadata) => m.mainboard.coil, chipColumns('coil')),
    ...lift((m: Sgb2Metadata) => m.mainboard.crystal, chipColumns('crystal')),
  ]),
];

export const CGB_CSV_COLUMNS: CsvColumn<CgbSubmission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: CgbSubmission) => s.metadata, [
    field('', 'color'),
    generate('', 'calendar_short', format.short.calendar),
    generate('', 'calendar', format.calendar),
    field('', 'year'),
    field('', 'month'),
    field('', 'week'),
    ...lift((m: CgbMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'number_pair'),
      field('mainboard', 'stamp'),
      field('mainboard', 'circled_letters'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
      // TODO: date_range
    ]),
    ...lift((m: CgbMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: CgbMetadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: CgbMetadata) => m.mainboard.amplifier, chipColumns('amplifier')),
    ...lift((m: CgbMetadata) => m.mainboard.regulator, chipColumns('regulator')),
    ...lift((m: CgbMetadata) => m.mainboard.crystal, chipColumns('crystal')),
  ]),
];

export const AGB_CSV_COLUMNS: CsvColumn<AgbSubmission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: AgbSubmission) => s.metadata, [
    field('', 'color'),
    generate('', 'calendar_short', format.short.calendar),
    generate('', 'calendar', format.calendar),
    field('', 'year'),
    field('', 'week'),
    ...lift((m: AgbMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'number_pair'),
      field('mainboard', 'stamp'),
      field('mainboard', 'circled_letters'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
    ]),
    ...lift((m: AgbMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: AgbMetadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: AgbMetadata) => m.mainboard.regulator, chipColumns('regulator')),
    ...lift((m: AgbMetadata) => m.mainboard.amplifier, chipColumns('amplifier')),
    ...lift((m: AgbMetadata) => m.mainboard.u4, chipColumns('u2')),
    ...lift((m: AgbMetadata) => m.mainboard.crystal, chipColumns('crystal')),
  ]),
];

export const AGS_CSV_COLUMNS: CsvColumn<AgsSubmission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: AgsSubmission) => s.metadata, [
    field('', 'color'),
    ...lift((m: AgsMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'number_pair'),
      field('mainboard', 'stamp'),
      field('mainboard', 'circled_letters'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
    ]),
    ...lift((m: AgsMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: AgsMetadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: AgsMetadata) => m.mainboard.amplifier, chipColumns('amplifier')),
    ...lift((m: AgsMetadata) => m.mainboard.u4, chipColumns('u2')),
    ...lift((m: AgsMetadata) => m.mainboard.u5, chipColumns('u5')),
    ...lift((m: AgsMetadata) => m.mainboard.crystal, chipColumns('crystal')),
  ]),
];

export const GBS_CSV_COLUMNS: CsvColumn<GbsSubmission>[] = [
  ...SUBMISSION_COLUMNS,
  ...lift((s: GbsSubmission) => s.metadata, [
    field('', 'color'),
    generate('', 'calendar_short', format.short.calendar),
    generate('', 'calendar', format.calendar),
    field('', 'year'),
    field('', 'week'),
    ...lift((m: GbsMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'number_pair'),
      field('mainboard', 'stamp'),
      field('mainboard', 'stamp_front'),
      field('mainboard', 'stamp_back'),
      field('mainboard', 'circled_letters'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
    ]),
    ...lift((m: GbsMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: GbsMetadata) => m.mainboard.work_ram, chipColumns('work_ram')),
    ...lift((m: GbsMetadata) => m.mainboard.u4, chipColumns('u2')),
    ...lift((m: GbsMetadata) => m.mainboard.u5, chipColumns('u5')),
    ...lift((m: GbsMetadata) => m.mainboard.u6, chipColumns('u6')),
    ...lift((m: GbsMetadata) => m.mainboard.crystal, chipColumns('crystal')),
  ]),
];

export const OXY_CSV_COLUMNS: CsvColumn<OxySubmission>[] =  [
  ...SUBMISSION_COLUMNS,
  ...lift((s: OxySubmission) => s.metadata, [
    field('', 'color'),
    ...lift((m: OxyMetadata) => m.mainboard, [
      field('mainboard', 'type'),
      field('mainboard', 'circled_letters'),
      generate('mainboard', 'calendar_short', format.short.calendar),
      generate('mainboard', 'calendar', format.calendar),
      field('mainboard', 'year'),
      field('mainboard', 'month'),
    ]),
    ...lift((m: OxyMetadata) => m.mainboard.cpu, chipColumns('cpu')),
    ...lift((m: OxyMetadata) => m.mainboard.u2, chipColumns('u2')),
    ...lift((m: OxyMetadata) => m.mainboard.u4, chipColumns('u4')),
    ...lift((m: OxyMetadata) => m.mainboard.u5, chipColumns('u5')),
  ]),
];

function lift<T, V>(f: (t: T) => V | null | undefined, columns: CsvColumn<V>[]): CsvColumn<T>[] {
  return columns.map(({name, get}) => ({
    name,
    get: (t: T) => {
      const value = f(t)
      return value === undefined || value === null
        ? ''
        : get(value)
    }
  }))
}

function field<T, K extends keyof T>(prefix: string, key: K): CsvColumn<T> {
  return {
    name: prefix ? `${prefix}_${key}` : key,
    get: (value) => value[key],
  }
}

function generate<T>(prefix: string, name: string, get: (value: T) => any): CsvColumn<T> {
  return {
    name: prefix ? `${prefix}_${name}` : name,
    get: (value: T) => {
      const result = get(value)
      if (result === null) {
        return '-'
      } else if (result === undefined) {
        return ''
      } else {
        return result
      }
    }
  }
}

function chipColumns(prefix: string): CsvColumn<Chip>[] {
  return [
    field(prefix, 'type'),
    field(prefix, 'label'),
    field(prefix, 'manufacturer'),
    generate(prefix, 'manufacturer_name', chip => format.manufacturer(chip.manufacturer || '')),
    generate(prefix, 'calendar_short', format.short.calendar),
    generate(prefix, 'calendar', format.calendar),
    field(prefix, 'year'),
    field(prefix,'month'),
    field(prefix, 'week'),
  ]
}