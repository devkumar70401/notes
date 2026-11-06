// c_motor.c
unsigned short calculate_motor_pwm(unsigned short current_speed, unsigned short boost) {
    return current_speed + boost;
}
