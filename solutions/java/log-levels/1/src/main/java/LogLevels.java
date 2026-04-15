public class LogLevels {
    
    public static String message(String logLine) {
        String[] parts = logLine.trim().split("\\s+", 2);
        StringBuilder rest = new StringBuilder();
            for (int i = 1; i < parts.length; i++) {
                if (i > 1) rest.append(" "); // Separator dazwischen
                rest.append(parts[i]);
            }
        return rest.toString();
    }

    public static String logLevel(String logLine) {
        String[] parts = logLine.split(" ");
        if (parts[0].equals("[ERROR]:")) {
            return "error";
        } else if (parts[0].equals("[WARNING]:")) {
            return "warning";
        } else {
            return "info";
        }
    }

    public static String reformat(String logLine) {
        String[] parts = logLine.trim().split("\\s+", 2);
        
        if (parts[0].equals("[ERROR]:")) {
            StringBuilder rest = new StringBuilder();
            for (int i = 1; i < parts.length; i++) {
                if (i > 1) rest.append(" "); // Separator dazwischen
                rest.append(parts[i]);
            }
            rest.append(" (error)");
            return rest.toString();
        } else if (parts[0].equals("[WARNING]:")) {
            StringBuilder rest = new StringBuilder();
            for (int i = 1; i < parts.length; i++) {
                if (i > 1) rest.append(" "); // Separator dazwischen
                rest.append(parts[i]);
            }
            rest.append(" (warning)");
            return rest.toString();
        } else {
            StringBuilder rest = new StringBuilder();
            for (int i = 1; i < parts.length; i++) {
                if (i > 1) rest.append(" "); // Separator dazwischen
                rest.append(parts[i]);
            }
            rest.append(" (info)");
            return rest.toString();
        }
    }
}
