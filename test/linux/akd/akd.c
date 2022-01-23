/** \file
 * \brief Example code for Simple Open EtherCAT master
 *
 * Usage : simple_test [ifname1]
 * ifname is NIC interface, f.e. eth0
 *
 * This is a minimal test.
 *
 * (c)Arthur Ketels 2010 - 2011
 */

#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <math.h>

#include "ethercat.h"

#define EC_TIMEOUTMON 500

char IOmap[4096];
OSAL_THREAD_HANDLE thread1;
int expectedWKC;
boolean needlf;
volatile int wkc;
boolean inOP;
uint8 currentgroup = 0;

int akd_setup(uint16 slave)
{
    int retval;
    uint8 u8val;
    uint16 u16val;
    //  uint32 u32val;

    printf("Setup AKD, PO2SO hook\n");

    retval = 0;

    // ---

    // // https://www.kollmorgen.com/en-us/developer-network/setup-ethercat-master-run-akd-%E2%80%9Cds402-cyclic-synchronous-position-mode%E2%80%9D/
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1C12, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // clear sm PDOs (0x1C12)
    //  printf("clear sm PDOs (0x1C12): %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1C13, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // clear sm PDOs (0x1C13)
    //  printf("clear sm PDOs (0x1C13): %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1A00, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear RX-PDO1
    //  printf("Clear RX-PDO1: %d\n", retval);
    //  u32val = 0x60410010;
    //  retval = ec_SDOwrite(slave, 0x1A00, 01, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Map status word to RX-PDO1
    //  printf("Map status word to RX-PDO1: %d\n", retval);
    //  u32val = 0x60640020;
    //  retval = ec_SDOwrite(slave, 0x1A00, 02, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Map position feedback to RX-PDO1
    //  printf("Map position feedback to RX-PDO1: %d\n", retval);
    //  u8val = 0x02;
    //  retval = ec_SDOwrite(slave, 0x1A00, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Set RX-PDO1 count to 2
    //  printf("Set RX-PDO1 count to 2: %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1A01, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear RX-PDO2
    //  printf("Clear RX-PDO2: %d\n", retval);
    //  u32val = 0x34700410;
    //  retval = ec_SDOwrite(slave, 0x1A01, 01, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Map analog input to RX-PDO2
    //  printf("Map analog input to RX-PDO2: %d\n", retval);
    //  u32val = 0x60FD0020;
    //  retval = ec_SDOwrite(slave, 0x1A01, 02, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Map digital inputs to RX-PDO2
    //  printf("Map digital inputs to RX-PDO2: %d\n", retval);
    //  u8val = 0x02;
    //  retval = ec_SDOwrite(slave, 0x1A01, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Set RX-PDO2 count to 2
    //  printf("Set RX-PDO2 count to 2: %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1A02, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear RX-PDO3
    //  printf("Clear RX-PDO3: %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1A03, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear RX-PDO4
    //  printf("Clear RX-PDO4: %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1600, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear TX-PDO1
    //  printf("Clear TX-PDO1: %d\n", retval);
    //  u32val = 0x60400010;
    //  retval = ec_SDOwrite(slave, 0x1600, 01, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Map control word to TX-PDO1
    //  printf("Map control word to TX-PDO1: %d\n", retval);
    //  u32val = 0x607A0020;
    //  retval = ec_SDOwrite(slave, 0x1600, 02, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Map position command to TX-PDO1
    //  printf("Map position command to TX-PDO1: %d\n", retval);
    //  u8val = 0x02;
    //  retval = ec_SDOwrite(slave, 0x1600, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Set TX-PDO1 count to 2
    //  printf("Set TX-PDO1 count to 2: %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1601, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear TX-PDO2
    //  printf("Clear TX-PDO2: %d\n", retval);
    //  u32val = 0x34700310;
    //  retval = ec_SDOwrite(slave, 0x1601, 01, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Map analog out to TX-PDO2
    //  printf("Map analog out to TX-PDO2: %d\n", retval);
    //  u32val = 0x60FE0120;
    //  retval = ec_SDOwrite(slave, 0x1601, 02, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Map digital out to TX-PDO2
    //  printf("Map digital out to TX-PDO2: %d\n", retval);
    //  u8val = 0x02;
    //  retval = ec_SDOwrite(slave, 0x1601, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Set TX-PDO2 count to 2
    //  printf("Set TX-PDO2 count to 2: %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1602, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear TX-PDO3
    //  printf("Clear TX-PDO3: %d\n", retval);
    //  u8val = 0x00;
    //  retval = ec_SDOwrite(slave, 0x1603, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear TX-PDO4
    //  printf("Clear TX-PDO4: %d\n", retval);
    //  u16val = 0x1600;
    //  retval = ec_SDOwrite(slave, 0x1C12, 01, FALSE, sizeof(u16val), &u16val, EC_TIMEOUTRXM); // Download PDO 0x1C12:01 index
    //  printf("Download PDO 0x1C12:01 index: %d\n", retval);
    //  u16val = 0x1601;
    //  retval = ec_SDOwrite(slave, 0x1C12, 02, FALSE, sizeof(u16val), &u16val, EC_TIMEOUTRXM); // Download PDO 0x1C12:02 index
    //  printf("Download PDO 0x1C12:02 index: %d\n", retval);
    //  u8val = 0x02;
    //  retval = ec_SDOwrite(slave, 0x1C12, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Download PDO 0x1C12 count
    //  printf("Download PDO 0x1C12 count: %d\n", retval);
    //  u16val = 0x1A00;
    //  retval = ec_SDOwrite(slave, 0x1C13, 01, FALSE, sizeof(u16val), &u16val, EC_TIMEOUTRXM); // Download PDO 0x1C13:01 index
    //  printf("Download PDO 0x1C13:01 index: %d\n", retval);
    //  u16val = 0x1A01;
    //  retval = ec_SDOwrite(slave, 0x1C13, 02, FALSE, sizeof(u16val), &u16val, EC_TIMEOUTRXM); // Download PDO 0x1C13:02 index
    //  printf("Download PDO 0x1C13:02 index: %d\n", retval);
    //  u8val = 0x02;
    //  retval = ec_SDOwrite(slave, 0x1C13, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Download PDO 0x1C13 count
    //  printf("Download PDO 0x1C13 count: %d\n", retval);
    //  u8val = 0x08;
    //  retval = ec_SDOwrite(slave, 0x6060, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Set OpMode to Cyclic Synchronous Position
    //  printf("Set OpMode to Cyclic Synchronous Position: %d\n", retval);
    //  u8val = 0x02;
    //  retval = ec_SDOwrite(slave, 0x60C2, 01, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Cycle time
    //  printf("Cycle time: %d\n", retval);
    //  u8val = 0xFD;
    //  retval = ec_SDOwrite(slave, 0x60C2, 02, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Cycle exponent
    //  printf("Cycle exponent: %d\n", retval);
    //  u32val = 0x00030001;
    //  retval = ec_SDOwrite(slave, 0x60FE, 02, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Digital output mask
    //  printf("Digital output mask: %d\n", retval);
    //  u32val = 0x00000010;
    //  retval = ec_SDOwrite(slave, 0x36E9, 00, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // FBUS.PARAM05, enable DS402 scaling
    //  printf("FBUS.PARAM05, enable DS402 scaling: %d\n", retval);
    //  u32val = 0x00000001;
    //  retval = ec_SDOwrite(slave, 0x6091, 01, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Motor revolutions
    //  printf("Motor revolutions: %d\n", retval);
    //  u32val = 0x00000001;
    //  retval = ec_SDOwrite(slave, 0x6091, 02, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Shaft revolutions
    //  printf("Shaft revolutions: %d\n", retval);
    //  u32val = 0x00100000;
    //  retval = ec_SDOwrite(slave, 0x6092, 01, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Feed (65536 counts/rev)
    //  printf("Feed (65536 counts/rev): %d\n", retval);
    //  u32val = 0x00000001;
    //  retval = ec_SDOwrite(slave, 0x6092, 02, FALSE, sizeof(u32val), &u32val, EC_TIMEOUTRXM); // Shaft revolutions
    //  printf("Shaft revolutions: %d\n", retval);

    // ---

    u8val = 0x00;
    ec_SDOwrite(slave, 0x1C12, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear SM PDO
    u8val = 0x00;
    ec_SDOwrite(slave, 0x1C13, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Clear SM PDO
    // u16val = 0x1701;
    u16val = 0x1724;                                                               // Allows CSP target position
    ec_SDOwrite(slave, 0x1C12, 01, FALSE, sizeof(u16val), &u16val, EC_TIMEOUTRXM); // Set fixed RXPDO map
    u8val = 0x01;
    ec_SDOwrite(slave, 0x1C12, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // One item mapped
    u16val = 0x1B01;
    // u16val = 0x1B24;                                                               // Read position from PL.FB instead of FB1.P
    ec_SDOwrite(slave, 0x1C13, 01, FALSE, sizeof(u16val), &u16val, EC_TIMEOUTRXM); // Set fixed TXPDO
    u8val = 0x01;
    ec_SDOwrite(slave, 0x1C13, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // One item mapped
    u8val = 0x08;
    ec_SDOwrite(slave, 0x6060, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Opmode - Cyclic Synchronous Position

    //
    u8val = 0x02;
    ec_SDOwrite(slave, 0x60C2, 01, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Interpolation time period
    u8val = 0xFD;
    ec_SDOwrite(slave, 0x60C2, 02, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // Interpolation time index

    // u8val = 0x90;
    // u8val = 0b10000;                                                             // Scale based on 0x6091 and 0x6092 https://www.kollmorgen.com/en-us/developer-network/position-scaling-akd-drive-ethercat-communication/
    u8val = 0;
    ec_SDOwrite(slave, 0x36E9, 00, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // FBUS.PARAM05
    // u8val = 0x01;
    // ec_SDOwrite(slave, 0x204C, 01, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // DS402.VELSCALENUM = 1
    // u8val = 0x0A;
    // ec_SDOwrite(slave, 0x204C, 02, FALSE, sizeof(u8val), &u8val, EC_TIMEOUTRXM); // DS402.VELSCALEDENOM = 10

    // ---

    printf("AKD slave %d set, retval = %d\n", slave, retval);
    return 1;
}

// Mapped by 0x1701 - ethercat manual p. 44
typedef struct PACKED
{
    int32_t TargetPosition;
    uint16_t ControlWord;
} akd_outputs_t;

// Mapped by 0x1b01 - ethercat manual p. 44
typedef struct PACKED
{
    int32_t PositionActualValue;
    uint16_t StatusWord;
} akd_inputs_t;

akd_outputs_t *out_ptr;
akd_inputs_t *in_ptr;

void simpletest(char *ifname)
{
    int i, j, oloop, iloop, chk;
    needlf = FALSE;
    inOP = FALSE;

    printf("Starting simple test\n");

    /* initialise SOEM, bind socket to ifname */
    if (ec_init(ifname))
    {
        printf("ec_init on %s succeeded.\n", ifname);
        /* find and auto-config slaves */

        if (ec_config_init(FALSE) > 0)
        {
            printf("%d slaves found and configured.\n", ec_slavecount);

            if ((ec_slavecount > 0))
            {
                int slc;
                for (slc = 1; slc <= ec_slavecount; slc++)
                {
                    // Kollmorgen AKD
                    if ((ec_slave[slc].eep_man == 0x0000006a) && (ec_slave[slc].eep_id == 0x00414b44))
                    {
                        printf("Found %s at position %d\n", ec_slave[slc].name, slc);
                        ec_slave[slc].PO2SOconfig = &akd_setup;
                    }
                }
            }

            ec_config_map(&IOmap);

            ec_configdc();

            // Send process data a few times to make outputs happy
            for (int i = 0; i < 15000; i++)
            {
                ec_send_processdata();
                ec_receive_processdata(EC_TIMEOUTRET);
            }

            printf("Slaves mapped, state to SAFE_OP.\n");
            /* wait for all slaves to reach SAFE_OP state */
            ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE * 4);

            oloop = ec_slave[0].Obytes;
            if ((oloop == 0) && (ec_slave[0].Obits > 0))
                oloop = 1;
            if (oloop > 8)
                oloop = 8;
            iloop = ec_slave[0].Ibytes;
            if ((iloop == 0) && (ec_slave[0].Ibits > 0))
                iloop = 1;
            if (iloop > 8)
                iloop = 8;

            in_ptr = (akd_inputs_t *)ec_slave[0].inputs;
            out_ptr = (akd_outputs_t *)ec_slave[0].outputs;

            printf("segments : %d : %d %d %d %d\n", ec_group[0].nsegments, ec_group[0].IOsegment[0], ec_group[0].IOsegment[1], ec_group[0].IOsegment[2], ec_group[0].IOsegment[3]);

            printf("Request operational state for all slaves\n");
            expectedWKC = (ec_group[0].outputsWKC * 2) + ec_group[0].inputsWKC;
            printf("Calculated workcounter %d\n", expectedWKC);
            ec_slave[0].state = EC_STATE_OPERATIONAL;
            /* send one valid process data to make outputs in slaves happy*/
            ec_send_processdata();
            ec_receive_processdata(EC_TIMEOUTRET);
            /* request OP state for all slaves */
            ec_writestate(0);
            chk = 200;
            /* wait for all slaves to reach OP state */
            do
            {
                ec_send_processdata();
                ec_receive_processdata(EC_TIMEOUTRET);
                ec_statecheck(0, EC_STATE_OPERATIONAL, 50000);
            } while (chk-- && (ec_slave[0].state != EC_STATE_OPERATIONAL));

            if (ec_slave[0].state == EC_STATE_OPERATIONAL)
            {
                printf("Operational state reached for all slaves.\n");
                inOP = TRUE;

                // If we've faulted, clear faults by setting clear fault flag high
                if ((in_ptr->StatusWord & 0b1000) > 0x0)
                {
                    out_ptr->ControlWord = 0x80; //clear errors, rising edge

                    do
                    {
                        printf("Wait for 6040 fault cleared, got %#04x\n", in_ptr->StatusWord);

                        ec_send_processdata();
                        ec_receive_processdata(EC_TIMEOUTRET);

                        osal_usleep(5000); // 0.5ms

                    } while ((in_ptr->StatusWord & 0b1000) > 0); // Fault flag is bit 4, wait for clear
                }

                // Shutdown
                out_ptr->ControlWord = 0x6;

                do
                {
                    printf("Wait for 6040 fault cleared again, got %#04x\n", in_ptr->StatusWord);

                    ec_send_processdata();
                    ec_receive_processdata(EC_TIMEOUTRET);

                    osal_usleep(5000); // 0.5ms

                } while ((in_ptr->StatusWord & 0b1) == 0); // ready to switch on, wait for it to be set

                // Switch on - this disengages the brake and "primes" the servo, but won't accept motion
                // commands yet.
                out_ptr->ControlWord = 0x7;

                do
                {
                    printf("Wait for 6040 switch on, got %#04x\n", in_ptr->StatusWord);

                    ec_send_processdata();
                    ec_receive_processdata(EC_TIMEOUTRET);

                    osal_usleep(5000); // 0.5ms

                } while ((in_ptr->StatusWord & 0b10) == 0); // switched on, wait for bit to be set

                int32_t current_turns = in_ptr->PositionActualValue;
                int32_t target_turns = current_turns + (2 * pow(2, 20));
                int32_t pos = current_turns;

                // Prevent motor from jumping on startup (I shit bricks lmao)
                out_ptr->TargetPosition = current_turns;

                // Enable operation - starts accepting motion comments
                out_ptr->ControlWord = 0xf;

                do
                {
                    printf("Wait for 6040 switch on, got %#04x\n", in_ptr->StatusWord);

                    ec_send_processdata();
                    ec_receive_processdata(EC_TIMEOUTRET);

                    osal_usleep(5000); // 0.5ms

                } while ((in_ptr->StatusWord & 0b100) == 0); // operation enable, wait for bit to be set

                printf("AKD state transitioned to Enable Operation\n");

                /* cyclic loop */
                for (i = 1; i <= 10000; i++)
                {
                    ec_send_processdata();
                    wkc = ec_receive_processdata(EC_TIMEOUTRET);

                    if (wkc >= expectedWKC)
                    {
                        if (pos < target_turns)
                        {
                            pos += 1000;
                        }

                        // omron_inputs_t * inputs = (omron_inputs_t)ec_slave[0].inputs;

                        out_ptr->TargetPosition = pos;

                        // TODO: Read 20 from FB1.PSCALE
                        // NOTE: FB1.PSCALE is not readable through ethercat, but defaults to 20.
                        // TODO: Make configurable
                        float turns = in_ptr->PositionActualValue / pow(2, 20);
                        float target_turns = out_ptr->TargetPosition / pow(2, 20);
                        // float target_turns = pos / pow(2, 20);

                        printf(
                            "Processdata cycle %4d, WKC %d, status %#04x, actual pos %d (%f turns), target pos %d (%f turns)",
                            i, wkc, in_ptr->StatusWord, in_ptr->PositionActualValue, turns, out_ptr->TargetPosition, target_turns);

                        // for(j = 0 ; j < oloop; j++)
                        // {
                        //     printf(" %2.2x", *(ec_slave[0].outputs + j));
                        // }

                        // printf(" I:");
                        // for(j = 0 ; j < iloop; j++)
                        // {
                        //     printf(" %2.2x", *(ec_slave[0].inputs + j));
                        // }
                        printf(" T:%" PRId64 "\r", ec_DCtime);
                        needlf = TRUE;
                    }
                    osal_usleep(5000);
                }
                inOP = FALSE;
            }
            else
            {
                printf("Not all slaves reached operational state.\n");
                ec_readstate();
                for (i = 1; i <= ec_slavecount; i++)
                {
                    if (ec_slave[i].state != EC_STATE_OPERATIONAL)
                    {
                        printf("Slave %d State=0x%2.2x StatusCode=0x%4.4x : %s\n",
                               i, ec_slave[i].state, ec_slave[i].ALstatuscode, ec_ALstatuscode2string(ec_slave[i].ALstatuscode));
                    }
                }
            }
            printf("\nRequest init state for all slaves\n");
            ec_slave[0].state = EC_STATE_INIT;
            /* request INIT state for all slaves */
            ec_writestate(0);
        }
        else
        {
            printf("No slaves found!\n");
        }
        printf("End simple test, close socket\n");
        /* stop SOEM, close socket */
        ec_close();
    }
    else
    {
        printf("No socket connection on %s\nExcecute as root\n", ifname);
    }
}

OSAL_THREAD_FUNC ecatcheck(void *ptr)
{
    int slave;
    (void)ptr; /* Not used */

    while (1)
    {
        if (inOP && ((wkc < expectedWKC) || ec_group[currentgroup].docheckstate))
        {
            if (needlf)
            {
                needlf = FALSE;
                printf("\n");
            }
            /* one ore more slaves are not responding */
            ec_group[currentgroup].docheckstate = FALSE;
            ec_readstate();
            for (slave = 1; slave <= ec_slavecount; slave++)
            {
                if ((ec_slave[slave].group == currentgroup) && (ec_slave[slave].state != EC_STATE_OPERATIONAL))
                {
                    ec_group[currentgroup].docheckstate = TRUE;
                    if (ec_slave[slave].state == (EC_STATE_SAFE_OP + EC_STATE_ERROR))
                    {
                        printf("ERROR : slave %d is in SAFE_OP + ERROR, attempting ack.\n", slave);
                        ec_slave[slave].state = (EC_STATE_SAFE_OP + EC_STATE_ACK);
                        ec_writestate(slave);
                    }
                    else if (ec_slave[slave].state == EC_STATE_SAFE_OP)
                    {
                        printf("WARNING : slave %d is in SAFE_OP, change to OPERATIONAL.\n", slave);
                        ec_slave[slave].state = EC_STATE_OPERATIONAL;
                        ec_writestate(slave);
                    }
                    else if (ec_slave[slave].state > EC_STATE_NONE)
                    {
                        if (ec_reconfig_slave(slave, EC_TIMEOUTMON))
                        {
                            ec_slave[slave].islost = FALSE;
                            printf("MESSAGE : slave %d reconfigured\n", slave);
                        }
                    }
                    else if (!ec_slave[slave].islost)
                    {
                        /* re-check state */
                        ec_statecheck(slave, EC_STATE_OPERATIONAL, EC_TIMEOUTRET);
                        if (ec_slave[slave].state == EC_STATE_NONE)
                        {
                            ec_slave[slave].islost = TRUE;
                            printf("ERROR : slave %d lost\n", slave);
                        }
                    }
                }
                if (ec_slave[slave].islost)
                {
                    if (ec_slave[slave].state == EC_STATE_NONE)
                    {
                        if (ec_recover_slave(slave, EC_TIMEOUTMON))
                        {
                            ec_slave[slave].islost = FALSE;
                            printf("MESSAGE : slave %d recovered\n", slave);
                        }
                    }
                    else
                    {
                        ec_slave[slave].islost = FALSE;
                        printf("MESSAGE : slave %d found\n", slave);
                    }
                }
            }
            if (!ec_group[currentgroup].docheckstate)
                printf("OK : all slaves resumed OPERATIONAL.\n");
        }
        osal_usleep(10000);
    }
}

int main(int argc, char *argv[])
{
    printf("SOEM (Simple Open EtherCAT Master)\nSimple test\n");

    if (argc > 1)
    {
        /* create thread to handle slave error handling in OP */
        //      pthread_create( &thread1, NULL, (void *) &ecatcheck, (void*) &ctime);
        // osal_thread_create(&thread1, 128000, &ecatcheck, (void*) &ctime);
        osal_thread_create_rt(&thread1, 128000, &ecatcheck, (void *)&ctime);
        /* start cyclic part */
        simpletest(argv[1]);
    }
    else
    {
        printf("Usage: simple_test ifname1\nifname = eth0 for example\n");
    }

    printf("End program\n");
    return (0);
}
