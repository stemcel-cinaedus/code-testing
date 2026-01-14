import java.util.Scanner;

public class week3 {
  public static void main(String[] args) {
    Scanner input = new Scanner(System.in);
    int[] numArray = new int[5];
    for (int i = 0; i<numArray.length; i++) {
      System.out.println("Input integer number " + (i+1));
      numArray[i] = input.nextInt();
  }
    int largest = 0;
    for (int i = 1; i<numArray.length; i++) {
      if (largest < numArray[i]) {
      largest = numArray[i];
    }
  }
    System.out.println("The largest number you input was " + largest);
  } 
}
