#include <iostream>
#include <cstdlib>
using namespace std;
int main(void){
	int sum = 0;
	int max = 16;
	for (int i = 0; i<max; i++){
		if (i%3 == 0 || i%5 == 0){
			sum += i;
			}
		}	
	cout << "The Sum of the first " << max << " 3 and 5 multiples is " << sum << endl;
	return EXIT_SUCCESS;
}
