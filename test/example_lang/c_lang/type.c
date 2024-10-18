#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>

// Перечисление для булевых значений
typedef enum {
    BOOL_FALSE = 0,
    BOOL_TRUE = 1
} bool_enum;

// определение структур для целых чисел
typedef struct {
    int8_t value;
} Int8;
typedef struct {
    int16_t value;
} Int16;
typedef struct {
    int32_t value;
} Int32;
typedef struct {
    int64_t value;
} Int64;

// определение структур для целых чисел

typedef struct {
    uint8_t value;
} UInt8;
typedef struct {
    int16_t value;
} UInt16;
typedef struct {
    int32_t value;
} UInt32;
typedef struct {
    int64_t value;
} UInt64;

// определение структур для не целых чисел
typedef struct {
    float value; // поле для хранения значения float32
} Float32;
typedef struct {
    double value; // поле для хранения значения float64
} Float64;
// Структура для булевого значения
typedef struct {
    bool_enum value; // Значение булевого типа
} Bool;


// функции для создания целых чисел
Int8 createInt8(int8_t value) {
    Int8 c;
    c.value = value;
    return c;
}
Int16 createInt16(int16_t value) {
    Int16 c;
    c.value = value;
    return c;
}
Int32 createInt32(int32_t value) {
    Int32 c;
    c.value = value;
    return c;
}
Int64 createInt64(int64_t value) {
    Int64 c;
    c.value = value;
    return c;
}

// функции для создания положительных чисел
UInt8 createUInt8(uint8_t value) {
    UInt8 c;
    c.value = value;
    return c;
}
UInt16 createUInt16(uint16_t value) {
    UInt16 c;
    c.value = value;
    return c;
}
UInt32 createUInt32(uint32_t value) {
    UInt32 c;
    c.value = value;
    return c;
}
UInt64 createUInt64(uint64_t value) {
    UInt64 c;
    c.value = value;
    return c;
}

// функция для создания Float32
Float32 createFloat32(float value) {
    Float32 f;
    f.value = value;
    return f;
}
// Функция для создания Float64
Float64 createFloat64(double value) {
    Float64 f;
    f.value = value;
    return f;
}

// Структура для представления строки
typedef struct {
    char *data; // Указатель на строку
    size_t length; // Длина строки
} String;

// Функция для создания строки
String createString(const char *initStr) {
    String str;
    str.length = strlen(initStr);
    str.data = malloc(str.length + 1);
    if (str.data != NULL) {
        strcpy(str.data, initStr);
    }
    return str;
}
// Функция для освобождения памяти строки
void freeString(String *str) {
    free(str->data);
    str->data = NULL;
}

// Функция для конкатенации двух строк
String concatStrings(const String *str1, const String *str2) {
    String result;
    result.length = str1->length + str2->length;
    result.data = malloc(result.length + 1); // +1 для нулевого символа

    if (result.data != NULL) {
        strcpy(result.data, str1->data); // Копируем первую строку
        strcat(result.data, str2->data); // Добавляем вторую строку
    }

    return result;
}
// Функция для сравнения двух строк
int compareStrings(const String *str1, const String *str2) {
    return strcmp(str1->data, str2->data); // Возвращает 0, если строки равны
}

// Функция для создания булевого значения
Bool createBool(const int value) {
    Bool b;
    b.value = (value != 0) ? BOOL_TRUE : BOOL_FALSE;
    return b;
}

// Функция для логического И двух булевых значений
Bool logicalAnd(Bool a, Bool b) {
    Bool result;
    result.value = (a.value == BOOL_TRUE && b.value == BOOL_TRUE) ? BOOL_TRUE : BOOL_FALSE;
    return result;
}
// Функция для логического ИЛИ двух булевых значений
Bool logicalOr(Bool a, Bool b) {
    Bool result;
    result.value = (a.value == BOOL_TRUE || b.value == BOOL_TRUE) ? BOOL_TRUE : BOOL_FALSE;
    return result;
}


// Функция для вывода булевого значения
void printBool(Bool b) {
    printf("%s\n", b.value == BOOL_TRUE ? "true" : "false");
}

