#include <unistd.h>

void 	ft_putnbr(int nb)
{
	long 	n;
	char 	c;

	n = (long) nb;
	if (n < 0)
	{
		write(1, "-", 1);
		n = -n;
	}
	if (n > 9)
		ft_putnbr((int) n / 10);
	c = n % 10 + '0';
	write(1, &c, 1);
}

int main(void)
{
	ft_putnbr(42);
	write(1, "\n", 1);
	ft_putnbr(10);
	write(1, "\n", 1);
	ft_putnbr(-42);
	write(1, "\n", 1);
	ft_putnbr(0);
	write(1, "\n", 1);
	return (0);
}
