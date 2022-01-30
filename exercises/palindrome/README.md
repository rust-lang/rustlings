In C, a simple palindrome checker would be following:

```c
int pallindrome(int arr[], int n) {
   int flag = 0;
   for(int i = 0, j=n-1; i< n/2, j>=n/2; i++, j--) {
      if(arr[i]!=arr[j]) {
         flag = 1;
         break;
      }
   }
   if (flag == 1)
   	return 0;
   else
   	return 1;
}
```

Implement the same code in Rust.