import java.util.List;
import java.util.Set;

class GottaSnatchEmAll {

    static Set<String> newCollection(List<String> cards) {
        return Set.copyOf(cards);
    }

    static boolean addCard(String card, Set<String> collection) {
        return collection.add(card);
    }

    static boolean canTrade(Set<String> myCollection, Set<String> theirCollection) {
        boolean a = !myCollection.containsAll(theirCollection);
        boolean b = !theirCollection.containsAll(myCollection);
        return a && b;
    }

    static Set<String> commonCards(List<Set<String>> collections) {
        if (collections == null || collections.isEmpty()) {
            return Set.of();
        }

        Set<String> common = Set.copyOf(collections.get(0));

        for (int i = 1; i < collections.size(); i++) {
            common = common.stream()
                           .filter(collections.get(i)::contains)
                           .collect(java.util.stream.Collectors.toUnmodifiableSet());
        }

        return common;
    }

     static Set<String> allCards(List<Set<String>> collections) {
        List<String> cards = new java.util.ArrayList<>();
        for (Set<String> set : collections) {
            for (String card : set) {
                if (!cards.contains(card)) {
                    cards.add(card);
                }
            }
        }
        return Set.copyOf(cards);
    }
}
