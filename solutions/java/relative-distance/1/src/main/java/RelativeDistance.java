import java.util.List;
import java.util.*;

class RelativeDistance {

    Map<String, List<String>> familyTree = new HashMap<>();
    
        RelativeDistance(Map<String, List<String>> familyTree) {
        for (Map.Entry<String, List<String>> entry : familyTree.entrySet()) {
            String parent = entry.getKey();
            List<String> children = entry.getValue();
            
            if (!this.familyTree.containsKey(parent)) {
                this.familyTree.put(parent, new ArrayList<>());
            }
            
            for (String child : children) {
                if (!this.familyTree.containsKey(child)) {
                    this.familyTree.put(child, new ArrayList<>());
                }
                
                this.familyTree.get(parent).add(child);
                this.familyTree.get(child).add(parent);
            }
            
            for (int i = 0; i < children.size(); i++) {
                for (int j = i + 1; j < children.size(); j++) {
                    String sibling1 = children.get(i);
                    String sibling2 = children.get(j);
                    
                    this.familyTree.get(sibling1).add(sibling2);
                    this.familyTree.get(sibling2).add(sibling1);
                }
            }
        }
    }

    int degreeOfSeparation(String personA, String personB) {
        if (personA.equals(personB)) {
            return 0;
        } else if (!familyTree.containsKey(personA)) {
            return -1;
        } else if (!familyTree.containsKey(personB)) {
            return -1;
        }

        Queue<String> queue = new LinkedList<>();
        Queue<Integer> distances = new LinkedList<>();
        HashSet<String> visited = new HashSet<>();

        queue.add(personA);
        distances.add(0);
        visited.add(personA);

        while (!queue.isEmpty()) {
            String currentPerson = queue.poll();
            int currentDistance = distances.poll();

            if (currentPerson.equals(personB)) {
                return currentDistance;
            }

            List<String> nachbarn = familyTree.get(currentPerson);

            for (String nachbar : nachbarn) {
                if (!visited.contains(nachbar)) {
                    visited.add(nachbar);
                    queue.add(nachbar);
                    distances.add(currentDistance + 1);
                }
            }
        }
        return -1;
    }
}
