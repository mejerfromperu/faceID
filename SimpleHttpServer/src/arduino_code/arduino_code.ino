#include "HUSKYLENS.h"
#include "SoftwareSerial.h"

HUSKYLENS huskylens;

// This code expects Huskylens to be connected using I2C-protocol

void printResult(HUSKYLENSResult result);

void setup() {
    Serial.begin(38400);
    Wire.begin();
    while (!huskylens.begin(Wire))
    {
        Serial.println(F("Begin failed!"));
        Serial.println(F("1.Please recheck the \"Protocol Type\" in HUSKYLENS (General Settings>>Protocol Type>>I2C)"));
        Serial.println(F("2.Please recheck the connection."));
        delay(100);
    }
}

void loop() {
    if (!huskylens.request()) Serial.println(F("Fail to request data from HUSKYLENS, recheck the connection!"));
    else if(!huskylens.available()) Serial.print(F("a"));
    else
    {
        
        while (huskylens.available())
        {
            HUSKYLENSResult result = huskylens.read();
            printResult(result);
        }    
    }
}

void printResult(HUSKYLENSResult result){
    Serial.print(result.ID == 1 || result.ID == 2 ? 'b' : 'a');
}