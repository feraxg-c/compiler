#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
typedef enum {BOOL_FALSE = 0,BOOL_TRUE = 1} bool_enum;
typedef struct {char *data; size_t length;} String;
typedef struct {int8_t value;} Int8;
typedef struct {int16_t value;} Int16;
typedef struct {int32_t value;} Int32;
typedef struct {int64_t value;} Int64;
typedef struct {uint8_t value;} UInt8;
typedef struct {int16_t value;} UInt16;
typedef struct {int32_t value;} UInt32;
typedef struct {int64_t value;} UInt64;
typedef struct {float value;} Float32;
typedef struct {double value;} Float64;
typedef struct {bool_enum value;} Bool;
Int8 createInt8(int8_t value) {Int8 c;c.value = value;return c;}
Int16 createInt16(int16_t value) {Int16 c;c.value = value;return c;}
Int32 createInt32(int32_t value) {Int32 c;c.value = value;return c;}
UInt8 createUInt8(uint8_t value) {UInt8 c;c.value = value;return c;}
UInt16 createUInt16(uint16_t value) {UInt16 c;c.value = value;return c;}
UInt32 createUInt32(uint32_t value) {UInt32 c;c.value = value;return c;}
Float32 createFloat32(float value) {Float32 f;f.value = value;return f;}
Float64 createFloat64(double value) {Float64 f;f.value = value;return f;}
String createString(const char *initStr) {
    String str;str.length = strlen(initStr);
    str.data = malloc(str.length + 1);
    if (str.data != NULL) {strcpy(str.data, initStr);}
    return str;}
void freeString(String *str) {free(str->data);str->data = NULL;}
Bool createBool(const int value) {
    Bool b;
    b.value = (value != 0) ? BOOL_TRUE : BOOL_FALSE;
    return b;}
void printBool(Bool b) {printf("%s\n", b.value == BOOL_TRUE ? "true" : "false");}
String concatStrings(const String *str1, const String *str2) {
    String result;
    result.length = str1->length + str2->length;
    result.data = malloc(result.length + 1);
    if (result.data != NULL) {
        strcpy(result.data, str1->data);
        strcat(result.data, str2->data);
    }return result;}
int compareStrings(const String *str1, const String *str2) {return strcmp(str1->data, str2->data);}
Bool logicalAnd(Bool a, Bool b) {
    Bool result;
    result.value = (a.value == BOOL_TRUE && b.value == BOOL_TRUE) ? BOOL_TRUE : BOOL_FALSE;
    return result;}
Bool logicalOr(Bool a, Bool b) {
    Bool result;
    result.value = (a.value == BOOL_TRUE || b.value == BOOL_TRUE) ? BOOL_TRUE : BOOL_FALSE;
    return result;}