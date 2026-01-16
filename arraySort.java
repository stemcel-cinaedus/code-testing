public class arraySort {
  public static void main(String[] args) {
    long startTime = System.nanoTime();
    int[] unsortedArray = {3,5,4,6,10,1,4,2,9,112,24,6,7,22,23,12,5,7,0,5,2,342,13,24634,213,3445,7,874434,123,4556,231,1224,22,4,6,12,34,16,89};
    //int[] sortedArray = new int[unsortedArray.length];
    for (int c=0; c < unsortedArray.length -1; c++) {
      for (int i=0; i < unsortedArray.length -1; i++) {
        if (unsortedArray[i] - unsortedArray[i+1] <= 0) {
          //sortedArray[i] = unsortedArray[i];
        }  else {
            int temp = unsortedArray[i+1];
            unsortedArray[i+1] = unsortedArray[i];
            unsortedArray[i] = temp; 
        }
        System.out.printf("%s%d%s%d%n", "The value of i is ", i, " and the value of the unsorted array is ", unsortedArray[i]);
      }
      System.out.print("This was run number " + c + " of the loop. \n\n\n");
    }
    System.out.println("Run time: " + (System.nanoTime() - startTime) + " nanoseconds.");  
  }
}
