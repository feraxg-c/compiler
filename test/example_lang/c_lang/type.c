#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>

// Структура для представления строки
typedef struct {
    char *data; // Указатель на строку
    size_t length; // Длина строки
} String;


// Определение структуры
typedef struct {
    char *value;
} CharLang;

typedef struct ListNode {
    void *data; // Указатель на данные
    struct ListNode *next; // Указатель на следующий узел
} ListNode;

typedef struct {
    ListNode *head; // Указатель на первый узел
    size_t size; // Размер списка
} List;


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




// Функции для создания целых чисел
Int8* createInt8(int8_t value) {
    Int8* c = (Int8*)malloc(sizeof(Int8)); // Выделение памяти
    if (c != NULL) {
        c->value = value;
    }
    return c; // Возврат указателя на выделенную память
}

Int16* createInt16(int16_t value) {
    Int16* c = (Int16*)malloc(sizeof(Int16));
    if (c != NULL) {
        c->value = value;
    }
    return c;
}

Int32* createInt32(int32_t value) {
    Int32* c = (Int32*)malloc(sizeof(Int32));
    if (c != NULL) {
        c->value = value;
    }
    return c;
}

Int64* createInt64(int64_t value) {
    Int64* c = (Int64*)malloc(sizeof(Int64));
    if (c != NULL) {
        c->value = value;
    }
    return c;
}

// Функции для создания положительных целых чисел
UInt8* createUInt8(uint8_t value) {
    UInt8* c = (UInt8*)malloc(sizeof(UInt8));
    if (c != NULL) {
        c->value = value;
    }
    return c;
}

UInt16* createUInt16(uint16_t value) {
    UInt16* c = (UInt16*)malloc(sizeof(UInt16));
    if (c != NULL) {
        c->value = value;
    }
    return c;
}

UInt32* createUInt32(uint32_t value) {
    UInt32* c = (UInt32*)malloc(sizeof(UInt32));
    if (c != NULL) {
        c->value = value;
    }
    return c;
}

UInt64* createUInt64(uint64_t value) {
    UInt64* c = (UInt64*)malloc(sizeof(UInt64));
    if (c != NULL) {
        c->value = value;
    }
    return c;
}

// Функции для создания Float32 и Float64
Float32* createFloat32(float value) {
    Float32* f = (Float32*)malloc(sizeof(Float32));
    if (f != NULL) {
        f->value = value;
    }
    return f;
}
Float64* createFloat64(double value) {
    Float64* f = (Float64*)malloc(sizeof(Float64));
    if (f != NULL) {
        f->value = value;
    }
    return f;
}

// Функции для освобождения памяти
void freeInt8(Int8* c) {
    free(c);
}

void freeInt16(Int16* c) {
    free(c);
}

void freeInt32(Int32* c) {
    free(c);
}

void freeInt64(Int64* c) {
    free(c);
}

void freeUInt8(UInt8* c) {
    free(c);
}

void freeUInt16(UInt16* c) {
    free(c);
}

void freeUInt32(UInt32* c) {
    free(c);
}

void freeUInt64(UInt64* c) {
    free(c);
}

void freeFloat32(Float32* f) {
    free(f);
}

void freeFloat64(Float64* f) {
    free(f);
}

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
Bool* createBool(const int value) {
    Bool* b = (Bool*)malloc(sizeof(Bool)); // Выделение памяти
    if (b != NULL) {
        b->value = (value != 0) ? BOOL_TRUE : BOOL_FALSE;
    }
    return b; // Возврат указателя на выделенную память
}

// Функция для логического И двух булевых значений
Bool* logicalAnd(Bool* a, Bool* b) {
    Bool* result = (Bool*)malloc(sizeof(Bool)); // Выделение памяти для результата
    if (result != NULL) {
        result->value = (a->value == BOOL_TRUE && b->value == BOOL_TRUE) ? BOOL_TRUE : BOOL_FALSE;
    }
    return result; // Возврат указателя на результат
}

// Функция для логического ИЛИ двух булевых значений
Bool* logicalOr(Bool* a, Bool* b) {
    Bool* result = (Bool*)malloc(sizeof(Bool)); // Выделение памяти для результата
    if (result != NULL) {
        result->value = (a->value == BOOL_TRUE || b->value == BOOL_TRUE) ? BOOL_TRUE : BOOL_FALSE;
    }
    return result; // Возврат указателя на результат
}

// Функция для вывода булевого значения
void printBool(Bool* b) {
    printf("%s\n", b->value == BOOL_TRUE ? "true" : "false");
}

// Функция для освобождения памяти
void freeBool(Bool* b) {
    free(b);
}

// Функция для создания и инициализации CharLang
CharLang *createCharLang(const char *initial_value) {
    CharLang *cl = (CharLang *)malloc(sizeof(CharLang));
    if (cl == NULL) {
        fprintf(stderr, "Ошибка: недостаточно памяти для создания CharLang.\n");
        return NULL;
    }
    cl->value = strdup(initial_value); // Копируем строку в новое место
    if (cl->value == NULL) {
        fprintf(stderr, "Ошибка: недостаточно памяти для значения.\n");
        free(cl);
        return NULL;
    }
    return cl;
}

// Функция для изменения значения
void setCharValue(CharLang *cl, const char *new_value) {
    if (cl->value != NULL) {
        free(cl->value); // Освобождаем память старого значения
    }
    cl->value = strdup(new_value); // Копируем новое значение
    if (cl->value == NULL) {
        fprintf(stderr, "Ошибка: недостаточно памяти для нового значения.\n");
    }
}

// Функция для получения значения
const char *getCharValue(const CharLang *cl) {
    return cl->value;
}

// Функция для освобождения памяти
void freeCharLang(CharLang *cl) {
    if (cl != NULL) {
        free(cl->value); // Освобождаем память значения
        free(cl);        // Освобождаем память структуры
    }
}

// Функция для создания списка
List* createList() {
    List *list = (List*)malloc(sizeof(List));
    if (list == NULL) {
        fprintf(stderr, "Failed to allocate memory for list\n");
        exit(EXIT_FAILURE);
    }
    list->head = NULL;
    list->size = 0;
    return list;
}

// Функция для добавления элемента в конец списка
void appendList(List *list, void *data) {
    ListNode *new_node = (ListNode*)malloc(sizeof(ListNode));
    if (new_node == NULL) {
        fprintf(stderr, "Failed to allocate memory for new node\n");
        exit(EXIT_FAILURE);
    }
    new_node->data = data;
    new_node->next = NULL;

    if (list->head == NULL) {
        // Если список пуст, новый узел становится головой
        list->head = new_node;
    } else {
        // Иначе добавляем узел в конец списка
        ListNode *current = list->head;
        while (current->next != NULL) {
            current = current->next;
        }
        current->next = new_node;
    }
    list->size++;
}

// Функция для освобождения памяти списка
void freeList(List *list) {
    ListNode *current = list->head;
    while (current != NULL) {
        ListNode *next = current->next;
        free(current); // Освобождаем узел
        current = next;
    }
    free(list); // Освобождаем сам список
}






