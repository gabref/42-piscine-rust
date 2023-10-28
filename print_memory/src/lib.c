#include <stdio.h>
#include <unistd.h>

void print_content_str(void *addr, int size, int limit)
{
	int 	i;
	char 	*str;

	i = 0;
	str = (char *) addr;
	while (--size && limit--)
	{
		if (*(str + i) && *(str + i) >= 32 && *(str + i) < 127)
			write(1, str + i, 1);
		else
			write(1, ".", 1);
		i++;
	}
}

void print_hex(unsigned char byte)
{
	char *hex_chars = "0123456789abcdef";
	write(1, &hex_chars[(byte & 0b11110000) >> 4], 1);
	write(1, &hex_chars[(byte & 0b00001111)], 1);
}

void print_content_hex(void *addr, int size, int limit)
{
	int 	i;
	char 	*str;

	i = 0;
	str = (char *) addr;

	while (--size && limit--)
	{
		print_hex(*(str + i));
		if (i++ % 2 != 0)
			write(1, " ", 1);
	}
	while (limit-- > 0)
	{
		write(1, "  ", 2);
		if (i++ % 2 != 0)
			write(1, " ", 1);
	}
	write(1, " ", 1);
}

void print_memory(void *addr)
{
	unsigned char *byte;
	int little_endian;

	byte = (unsigned char *) &addr;
	little_endian = 7;
	while (little_endian >= 0)
		print_hex(*(byte + little_endian--));
	write(1, ": ", 2);
}

void	*ft_print_memory(void *addr, unsigned int size)
{
	int i;
	int limit;

	limit = 16;
	i = 0;
	while (i < size)
	{
		print_memory(addr + i);
		print_content_hex(addr + i, size - i, limit);
		print_content_str(addr + i, size - i, limit);
		
		if (i < size)
			write(1, "\n", 1);
		i += 16;
	}
	return (addr);
}

int main(void)
{
	char *str = "Bonjour les aminches\t\n\tc\n est fou\ttout\tce qu on peut faire avec\t\n\tprint_memory\n\n\n\tlol.lol\n \0";
	int size = -1;
	while (str[++size]);
	size++; // because of additional null at end of string

	ft_print_memory(str, size);
}
