
#include <stdio.h>
void ft_ultimate_div_mod(int *a, int *b)
{
	*a = *a / *b;
	*b = (*a * *b) % *b;
}

int main(void)
{
	int a;
	int b;

	a = 10;
	b = 5;
	printf("%d and %d. ", a, b);
	ft_ultimate_div_mod(&a, &b);
	printf("div: %d. mod: %d\n", a, b);
	return (0);
}
