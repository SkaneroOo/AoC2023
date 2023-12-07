import java.io.File;
import java.util.HashMap;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        HashMap<String, Integer> maxColors = new HashMap<String, Integer>();
        maxColors.put("red", 12);
        maxColors.put("green", 13);
        maxColors.put("blue", 14);
        int total = 0;
        int minimum = 0;
        try {
            File input = new File("./input.txt");
            Scanner reader = new Scanner(input);
            while (reader.hasNextLine()) {
                HashMap<String, Integer> colors = new HashMap<String, Integer>();
                String line = reader.nextLine();
                String[] parts = line.split(": ");
                String game = parts[0];
                int gameNumber = Integer.parseInt(game.substring(5));
                String[] scores = parts[1].split("; ");
                for (String score : scores) {
                    String[] colorAndCount = score.split(", ");
                    for (String color : colorAndCount) {
                        String[] colorParts = color.split(" ");
                        colors.put(colorParts[1], Math.max(colors.getOrDefault(colorParts[1], 0), Integer.parseInt(colorParts[0])));
                    }
                }
                boolean can_add = true;
                int temp = 1;
                for (String color : colors.keySet()) {
                    temp *= colors.get(color);
                    if (colors.get(color) > maxColors.get(color)) {
                        can_add = false;
                    }
                }
                minimum += temp;
                if (can_add) {
                    total += gameNumber;
                }
            }
            System.out.println(total);
            System.out.println(minimum);
            reader.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}