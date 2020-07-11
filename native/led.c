#include <stdio.h>    // for printf
#include <fcntl.h>    // for open
#include <sys/mman.h> // for mmap
#include <unistd.h>

#define GPCLR0 0x28
#define GPSET0 0x1C
#define GPLEV0 0x34

/* 
    Sprawdza stan pina o numerze 23, ustawia stan na wysoki po czym zmienia na niski

    https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2835/BCM2835-ARM-Peripherals.pdf
    https://www.cs.uaf.edu/2016/fall/cs301/lecture/11_09_raspberry_pi.html
    https://elinux.org/RPi_GPIO_Code_Samples
 */
int main()
{
    int fd = open("/dev/gpiomem", O_RDWR);
    if (fd < 0)
    {
        printf("Error opening /dev/gpiomem");
        return -1;
    }

    unsigned int *gpio = (unsigned int *)mmap(0, 4096,
                                              PROT_READ + PROT_WRITE, MAP_SHARED,
                                              fd, 0);

    int gpio_num = 23;

    // offset w bajtach pomiędzy kolejnymi elementami w tablicy
    // 32 bity = 8 * 4 bajty
    int u32_offset = 4;

    int FSEL_SHIFT = (gpio_num) / 10;

    // każdy pin ma przypisane 3 bity
    // 000 -> input
    // 001 -> output
    // 010 i wyżej -> alternate functions (zależne od numeru pinu)
    //
    // więcej w rozdziale 6.2
    gpio[FSEL_SHIFT] &= ~(7 << (((gpio_num) % 10) * 3)); // zawsze przed ustawieniem na output musimy ustawić na input
    gpio[FSEL_SHIFT] |= (1 << (((gpio_num) % 10) * 3));  // output

    // GPLEV0 piny 0 - 31, ten kod nie obsłuży pin > 31
    // Odczytuje stan pina gpio_num poprzez odczytanie bitu gpio_num rejestru GPLEV0
    int state = (gpio[GPLEV0 / u32_offset] >> gpio_num) & 1;
    printf("status is %i\n", state);

    while (1==1)
    {
        sleep(1);

        gpio[GPSET0 / u32_offset] |= 1 << gpio_num;
        printf("set to high\n");

        sleep(1);

        gpio[GPCLR0 / u32_offset] |= 1 << gpio_num;
        printf("set to low\n");
    }

    return 0;
}
