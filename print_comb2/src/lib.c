#include <unistd.h>

void ft_print_comb2(void)
{
	int 	i;
	int 	j;
	char	c;

	i = 0;
	while (i <= 98)
	{
		j = i + 1;
		while (j <= 99)
		{
			c = i / 10 + 48;
			write(1, &c, 1);
			c = i % 10 + 48;
			write(1, &c, 1);
			write(1, " ", 1);
			c = j / 10 + 48;
			write(1, &c, 1);
			c = j % 10 + 48;
			write(1, &c, 1);
			if (i != 98)
			{
				write(1, ", ", 2);
			}
			j++;
		}
		i++;
	}
}

int main(void)
{
	ft_print_comb2();
	return (0);
}
