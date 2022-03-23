#include <stdio.h>
#include <fcntl.h>     //open


/*
 * gpio_test on
 * gpio_test off
 */

int main(int argc, char **argv)
{
	int fd;
	int val = 0;
	if(argc != 2)
	{
		printf("Usage: \n");
		printf("%s: <on|off>\n", argv[0]);     //<> must have on | off
		return 0;
	}
	
	fd = open("/dev/gpio_chardev",O_RDWR);
	if( fd < 0 )
	{
		printf("can not open\n");
        return -1;
	}
	
	if( strcmp(argv[1],"on") == 0 )
	{
		char tmp = 1;
		printf("led on\n");
		write(fd, &tmp, 1);
	}
	else if( strcmp(argv[1],"off") == 0 )
	{
		char tmp = 2;
		printf("led off\n");
		write(fd, &tmp, 1);
	}
	
}

