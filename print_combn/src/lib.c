#include <unistd.h>

void print_combn(int arr[], char n, int index)
{
	char 	i;
	int 	idx;
	int 	size;

	if (index == 0)
	{
		size = -1;
		while (arr[size++]);
		idx = 0;
		while (size != idx)
		{
			write(1, arr + size - 1 - idx++, 1);
		}
		if (arr[size - 2] != 59 - size)
			write(1, ", ", 2);
		return ;
	}
	i = n;
	while (i <= (58 - index))
	{
		arr[index - 1] = i;
		print_combn(arr, i + 1, index - 1);
		i++;
	}
}

void ft_print_combn(int n)
{
	int arr[n + 1];

	arr[n] = 0;
	print_combn(arr, '0', n);
}

int main(void)
{
	ft_print_combn(3);
	write(1, "\n", 1);
	ft_print_combn(2);
	write(1, "\n", 1);
	ft_print_combn(1);
	write(1, "\n", 1);
	return (0);
}
