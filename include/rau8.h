#pragma once

#include <stdbool.h>
#include <stdint.h>

bool rau8_valid(const char* str);
int64_t rau8_bytes(const char* str);
int64_t rau8_scalar_values(const char* str);
int64_t rau8_grapheme_clusters(const char* str);