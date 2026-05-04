public class LogLine {
    private String logLine;
    
    public LogLine(String logLine) {
        this.logLine = logLine;
    }

    public LogLevel getLogLevel() {
        String code = logLine.substring(1, 4);
        switch (code) {
            case "TRC": return LogLevel.TRACE;
            case "DBG": return LogLevel.DEBUG;
            case "INF": return LogLevel.INFO;
            case "WRN": return LogLevel.WARNING;
            case "ERR": return LogLevel.ERROR;
            case "FTL": return LogLevel.FATAL;
            default: return LogLevel.UNKNOWN;
        }
    }

    public String getOutputForShortLog() {
        String code = logLine.substring(1, 4);
        String message = logLine.substring(7);
        switch (code) {
            case "TRC": return "1:" + message;
            case "DBG": return "2:" + message;
            case "INF": return "4:" + message;
            case "WRN": return "5:" + message;
            case "ERR": return "6:" + message;
            case "FTL": return "42:" + message;
            default: return "0:" + message;
        }
    }
}
