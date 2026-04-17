class NeedForSpeed {
    private int speed;
    private int batteryDrain;
    private int metersDriven;
    private int battery;
    
    NeedForSpeed(int speed, int batteryDrain) {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
        this.metersDriven = 0;
        this.battery = 100;
    }

    public boolean batteryDrained() {
        return battery < batteryDrain;
    }

    public int distanceDriven() {
        return metersDriven;
    }

    public void drive() {
        if (battery - batteryDrain < 0) {
            return;
        }
        battery -= batteryDrain;
        metersDriven += speed;
    }

    public static NeedForSpeed nitro() {
        return new NeedForSpeed(50 ,4);
    }
}

class RaceTrack {
    private int distance;
    RaceTrack(int distance) {
        this.distance = distance;
    }

    public boolean canFinishRace(NeedForSpeed car) {
        while (!car.batteryDrained()) {
            car.drive();
        }
        return car.distanceDriven() >= distance;
    }
}
