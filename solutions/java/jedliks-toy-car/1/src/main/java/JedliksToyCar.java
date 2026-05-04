public class JedliksToyCar {
        int batteries = 100;
        int meters;
    public static JedliksToyCar buy() {
        return new JedliksToyCar();  
    }

    public String distanceDisplay() {
        return String.format("Driven %d meters", meters);
    }

    public String batteryDisplay() {
        if (batteries == 0) {
            return "Battery empty";
        }
        return String.format("Battery at %d%%", batteries);
    }

    public void drive() {
        if (batteries > 0) {   
        meters += 20;
        batteries -= 1;
        }
    }
}
