import java.util.Random;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

class CaptainsLog {

    private static final char[] PLANET_CLASSES = new char[]{'D', 'H', 'J', 'K', 'L', 'M', 'N', 'R', 'T', 'Y'};

    private Random random;

    CaptainsLog(Random random) {
        this.random = random;
    }

    char randomPlanetClass() {
        List<Character> buchstaben = List.of('D', 'H', 'J', 'K', 'L', 'M', 'N', 'R', 'T', 'Y');
        int index = random.nextInt(buchstaben.size());

        return buchstaben.get(index);
    }

    String randomShipRegistryNumber() {
        int zahl = random.nextInt(10000 - 1000) + 1000;
        return "NCC-" + zahl;
    }

    double randomStardate() {
        return 41000 +random.nextDouble() * (42000 - 41000);
    }
}
