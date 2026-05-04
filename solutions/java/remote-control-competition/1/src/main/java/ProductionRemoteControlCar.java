class ProductionRemoteControlCar implements RemoteControlCar, Comparable<ProductionRemoteControlCar> {
    private int distance = 0;
    private int victories = 0;
    
    public void drive() {
        distance += 10;
    }

    public int getDistanceTravelled() {
        return distance; 
    }

    public int getNumberOfVictories() {
        return victories;
    }

    public void setNumberOfVictories(int numberOfVictories) {
        this.victories = numberOfVictories;
    }

    public int compareTo(ProductionRemoteControlCar other) {
        return Integer.compare(other.victories, this.victories);
    }
}
