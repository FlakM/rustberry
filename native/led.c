#include <stdio.h>    // for printf
#include <fcntl.h>    // for open
#include <sys/mman.h> // for mmap
#include <unistd.h>

int main()
{
    int fdgpio = open("/dev/gpiomem", O_RDWR);
    if (fdgpio < 0)
    {
        printf("Error opening /dev/gpiomem");
        return -1;
    }

    unsigned int *gpio = (unsigned int *)mmap(0, 4096,
                                              PROT_READ + PROT_WRITE, MAP_SHARED,
                                              fdgpio, 0);
    printf("mmap'd gpiomem at pointer %p\n", gpio);

    int gpio_num = 18;

    // 32 bits
    int register_size = 4; 


    int SET_MODE = (0x00 + gpio_num) / 10;
    
    // https://librambutan.readthedocs.io/en/latest/lang/cpp/compoundbitwise.html#compound-bitwise-or
    // Each pin has a 3-bit field, with 000 meaning input, 001 meaning output, and 010 and higher meaning "alternate functions" specific to each pin
    // więcej w rozdziale 6.2
    gpio[SET_MODE] &= ~(7<<(((gpio_num)%10)*3)); // set input mode before we can set output mode
    gpio[SET_MODE] |=  (1<<(((gpio_num)%10)*3)); // set output mode


    // GPLEV0 pins 0 - 31
    int GPLEV0 = 0x34 / register_size;

    // Read pin gpio_num, by accessing bit gpio_num of GPLEV0
    int state = (gpio[GPLEV0] >> gpio_num) & 1;


    printf("status is %i\n", state);


    int GPSET0 = 0x1C / register_size;
    gpio[GPSET0]|=1<<gpio_num;
    printf("set to high\n");

    sleep(3);

    int GPCLR0 = 0x28 / register_size;
    gpio[GPCLR0]|=1<<gpio_num;
    printf("set to low\n");


    return 0;
}
