class SqueakyClean {
    static String clean(String identifier) {
        StringBuilder result = new StringBuilder();
        boolean toUpper = false;
        String replace = identifier.replace(" ", "_")
                                .replace("4", "a")
                                .replace("3", "e")
                                .replace("0", "o")
                                .replace("1", "l")
                                .replace("7", "t");
        for (char c : replace.toCharArray()){
            if (!Character.isLetter(c) && c != '_' && c != '-') {
                continue;
            }
            if (c == '-') {
                toUpper = true;
            } else {
                if (toUpper) {
                    result.append(Character.toUpperCase(c));
                    toUpper = false;
                } else {
                    result.append(c);
                }
            }
        }
        return result.toString();
    }
}
