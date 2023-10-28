#include <stdio.h>
void ft_swap(int *a, int *b)
{
	int t = *a;
	*a = *b;
	*b = t;
}

int main(void)
{
	int a;
	int b;

	a = 10;
	b = 20;
	printf("%d - %d\n", a, b);
	ft_swap(&a, &b);
	printf("%d - %d\n", a, b);
	return (0);
}
