public class brokenSort {
  public static void main(String[] args) {
    long startTime = System.nanoTime();
    int numArray[] = {4,5,15,5,71,1,15,61,617,868,2,112,346,572,13,546,524,524,7583,42,15,46,586,13,51,3135,32,12,13,12,12,13,117,7889,89,0,1,1,6,43};
    for (int i=1; i<numArray.length-1; i++) {
        System.out.println("Next iteration of outer loop :3");
        int c = i;
        //There is an issue somewhere with these while statements that only lets each one run once, I am unsure what it is.
        //The if statements probably aren't necessary, I don't know how loops work in Java though. If it just goes to the code after the while loop after the conditions aren't met, then I can delete the if loops entirely.
        //I don't think the if loops are causing any logical errors, but it would still be more efficient to remove them.
        while (c < numArray.length -1 && numArray[c] > numArray[c+1]) {
          int temp = numArray[c+1];
          numArray[c+1] = numArray[c];
          numArray[c] = temp;
          c++;
          System.out.println("Moved a number up");
        }
      }
    for (int i=numArray.length-1; i>=1; i--) {
      int c = i;
      while (numArray[c] < numArray[c-1]) {
           int temp = numArray[c-1];
           numArray[c-1] = numArray[c];
           numArray[c] = temp;
           if (c>1) {
           c--;
           }
           System.out.println("Moved a number down");
        }
    }  
  for (int s=0; s < numArray.length; s++) {
    System.out.println("The value at index " + s + " is " + numArray[s]);
  }
  System.out.println("Total time: " + (System.nanoTime() - startTime));
  }
}
