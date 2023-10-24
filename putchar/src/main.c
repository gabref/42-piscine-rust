#include <unistd.h>

void ft_putchar(char c) {
	write(1, &c, 1);
}

int main(void)
{
	char str[6] = { 'h', 'e', 'l', 'l', 'o', '\n' };
	int i;

	i = 0;
	while (i < 6)
	{
		ft_putchar(str[i++]);
	}

	return (0);
}
