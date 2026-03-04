export const APP_VERSION = "1.0.2.51";
export const APP_TITLE = `YUNFEI自动压缩_${APP_VERSION}`;

export type TimeGateOption = "last24h" | "last7d" | "customDate";
export type ScanIntervalOption = "1" | "5" | "15" | "60";
export type MountCheckIntervalOption = "3" | "5" | "10";
export type CodecOption = "h264" | "h265";
export type QualityPreset = "spaceSaving" | "balanced" | "highQuality";
export type OutputMode = "overwrite" | "outputFolder" | "suffix";

export interface QueueEntry {
  id: string;
  name: string;
  sizeBytes: number;
  estimatedBytes: number | null;
}

export interface FrontendState {
  inputPath: string;
  extraInputPaths: string[];
  outputPath: string;
  includeSubfolders: boolean;
  timeGate: TimeGateOption;
  customDate: string | null;
  scanInterval: ScanIntervalOption;
  immediateCompress: boolean;
  codec: CodecOption;
  qualityPreset: QualityPreset;
  outputMode: OutputMode;
  suffixText: string;
  keywords: string[];
  launchAtLogin: boolean;
  keepAlive: boolean;
  mountMonitorEnabled: boolean;
  mountInterval: MountCheckIntervalOption;
  ffmpegPath: string;

  scanEnabled: boolean;
  statusText: string;
  mountStatus: string;
  logs: string[];
  queueItems: QueueEntry[];
  batchTotal: number;
  batchCompleted: number;
  currentFileName: string;
  currentFileProgress: number;
  currentFileElapsed: number;
  currentFileRemaining: number;
  currentFileEstimatedTotal: number;
  lastCompletedName: string;
  ffmpegVersionShort: string;
  ffmpegVersionFull: string;
  ffmpegEncoderList: string[];
  ffmpegEncoderError: string;
  hardwareStatusText: string;
  hardwareStatusHint: string;
  currentEncoderText: string;
  hardwareAvailableH264: boolean;
  hardwareAvailableH265: boolean;
  isDownloadingFfmpeg: boolean;
  isCompressing: boolean;

  statusDisplay: string;
  queueProgress: string;
  draftKeyword: string;
  draftSuffix: string;
}

export const defaultState: FrontendState = {
  inputPath: "",
  extraInputPaths: [],
  outputPath: "",
  includeSubfolders: false,
  timeGate: "last24h",
  customDate: null,
  scanInterval: "5",
  immediateCompress: false,
  codec: "h265",
  qualityPreset: "balanced",
  outputMode: "overwrite",
  suffixText: "_压缩",
  keywords: ["带字幕"],
  launchAtLogin: false,
  keepAlive: true,
  mountMonitorEnabled: true,
  mountInterval: "5",
  ffmpegPath: "",

  scanEnabled: false,
  statusText: "已停止",
  mountStatus: "未检测",
  logs: [],
  queueItems: [],
  batchTotal: 0,
  batchCompleted: 0,
  currentFileName: "",
  currentFileProgress: 0,
  currentFileElapsed: 0,
  currentFileRemaining: 0,
  currentFileEstimatedTotal: 0,
  lastCompletedName: "",
  ffmpegVersionShort: "未配置",
  ffmpegVersionFull: "",
  ffmpegEncoderList: [],
  ffmpegEncoderError: "",
  hardwareStatusText: "硬件编码状态未知",
  hardwareStatusHint: "",
  currentEncoderText: "",
  hardwareAvailableH264: false,
  hardwareAvailableH265: false,
  isDownloadingFfmpeg: false,
  isCompressing: false,

  statusDisplay: "",
  queueProgress: "",
  draftKeyword: "",
  draftSuffix: "_压缩",
};
