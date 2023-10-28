#include <stdio.h>
void ft_ft(int *nbr)
{
	*nbr = 42;
}

int main(void)
{
	int n;

	ft_ft(&n);
	printf("value: %d\n", n);
	return (0);
}
