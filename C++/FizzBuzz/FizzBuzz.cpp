#include <cstdio>
#include <cstdlib>
#include <iostream>
using namespace std;
int main(void){
	int max = 2048;
	for (int i=0; i<=max; i++){
		if (i%3 == 0 && i%5 == 0){
				cout << "FizzBuzz";
		}
		else if (i%3 == 0){
				cout << "Fizz";
			}
		else if (i%5 == 0){
				cout << "Buzz";
		}
		else{
				cout << i;
		}
			
		cout << endl;
	}

	return EXIT_SUCCESS;
}
