public class arraySort {
  public static void main(String[] args) {
    int[] unsortedArray = {3,5,4,6,10,1,4,2,9};
    int[] sortedArray = new int[unsortedArray.length];
    for (int c=0; c < unsortedArray.length -1; c++) {
      for (int i=0; i < unsortedArray.length -1; i++) {
        if (unsortedArray[i] - unsortedArray[i+1] <= 0) {
          sortedArray[i] = unsortedArray[i];
        }  else {
            int temp = unsortedArray[i+1];
            unsortedArray[i+1] = unsortedArray[i];
            unsortedArray[i] = temp; 
        }
        System.out.printf("%s%d%s%d%s%d%n", "The value of i is ", i, " and the value of the unsorted array is ", unsortedArray[i], " and the value of the sorted array is ", sortedArray[i]);
      }
      System.out.print("This was run number " + c + " of the loop. \n\n\n");
    }    
  }
}
