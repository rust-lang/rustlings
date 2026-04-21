class CalculatorConundrum {
    public String calculate(int operand1, int operand2, String operation) {
        switch (operation) {
            case null : throw new IllegalArgumentException("Operation cannot be null");
            case "" : throw new IllegalArgumentException("Operation cannot be empty");
            case "+" : return operand1 + " + " + operand2 + " = " + (operand1 + operand2);
            case "*" : return operand1 + " * " + operand2 + " = " + (operand1 * operand2);
            case "/" :
                try {
                    return operand1 + " / " + operand2 + " = " + (operand1 / operand2);
                } catch (ArithmeticException e) {
                    throw new IllegalOperationException("Division by zero is not allowed", e);
                }
            default: throw new IllegalOperationException(String.format("Operation '%s' does not exist", operation));

        }
    }
}
