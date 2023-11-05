#!/usr/bin/env python
# coding: utf-8

# In[3]:


import numpy as np
import pandas as pd


# ## Читаємо дані та представляємо їх у правильному вигляді

# In[4]:


df_table = pd.read_csv(r'./table_04.csv')
length_table = 20


# In[5]:


df_table


# In[6]:


df_probabilites = pd.read_csv(r'./prob_04.csv')


# In[7]:


df_probabilites.head()


# Як бачимо, маємо деякі проблеми з форматом даних. Тож задамо до обоїх датасетів відповідні виправлені списки:

# In[8]:


distribution_messages = [0.09,0.09,0.09,0.09,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04,0.04]


# In[9]:


distribution_keys = [0.05 for _ in range(length_table)]


# In[10]:


matrix = df_table.values
missing_row = np.array([3, 11, 17, 19, 12, 6, 2, 15, 13, 14, 1, 18, 5, 0, 16, 8, 10, 7, 4, 9])
encryption_table = np.vstack((missing_row, matrix))
pd.DataFrame(encryption_table)


# In[11]:


#Obtaining a set of pairs from keys and messages for each ciphertext.

index_pairs = {cryptotext: [] for cryptotext in range(length_table)}

for index_key in range(length_table):
    for index_message in range(length_table):
        
        text = encryption_table[index_key][index_message]
        index_pairs[text].append((index_key, index_message))

#Showing a set of pairs
pd.DataFrame.from_dict(index_pairs)

#index_pairs


# ### Формуємо детерміністичні вирішуючі функції
# 1) Обчислюємо $P(C):$
# $$\forall C: P(C) = \sum_{(M,k):E_{k}(M) = C}P(M,k)$$

# In[12]:


probabilites_ciphertext = []
for crypto_index in index_pairs:
    
    probability_products = []
    list_of_pairs = index_pairs[crypto_index]
    
    for pair in list_of_pairs:
        
        index_key, index_message = pair
        product = distribution_keys[index_key]*distribution_messages[index_message]
        probability_products.append(product)
    
    probability = sum(probability_products)
    probabilites_ciphertext.append(probability)

pd.DataFrame(probabilites_ciphertext)        


#  2) Обчислюємо $P(M, C):$
# $$\forall (M,C): P(M,C) = \sum_{k:E_{k}(M) = C}P(M,k)$$

# In[13]:


probabilites_ciphertext_messages = [[0]*length_table for _ in range(length_table)]

for crypto_index in range(length_table):
    for message_index in range(length_table):
        keys = []
        for pair in index_pairs[crypto_index]:
            if pair[-1] == message_index:
                keys.append(pair[0])
        keys_distributed = [distribution_keys[index_key] for index_key in keys]
        product = distribution_messages[message_index]*sum(keys_distributed)
        probabilites_ciphertext_messages[message_index][crypto_index] = product

pd.DataFrame(probabilites_ciphertext_messages)
    


# 3) Обчислюємо $P(M|C) = \dfrac{P(M, C)}{P(C)}$

# In[14]:


condtitional_probabilities = pd.DataFrame(index = range(length_table), columns = range(length_table))
for message in range(length_table):
    for cipher_text in range(length_table):
        condtitional_probabilities.at[message, cipher_text] = (
        probabilites_ciphertext_messages[message][cipher_text]/probabilites_ciphertext[cipher_text])

condtitional_probabilities


# 4) Вибираємо максимальні значення та утворюємо детерміністичну вирішуючу функцію, що і буде баєсівською, відповідно до теореми. 

# In[15]:


# Finding maximum for all rows
max_values = condtitional_probabilities.max()
df = condtitional_probabilities

values_to_find = max_values
j_indices = [_ for _ in range(length_table)] 

result_df = pd.DataFrame(index=pd.MultiIndex.from_tuples(zip(values_to_find, j_indices), names=['Max_Value', 'Cipher_text_index']))

def find_index(value, j_index):
    return df.loc[df.iloc[:, j_index] == value].index[0]

message_indexes = [find_index(value, j_index) for value, j_index in zip(values_to_find, j_indices)]
maximazied_values = {'Ciphertexts': [index for index in range(length_table)],
                     'Messages': message_indexes}

decision_function = pd.DataFrame(maximazied_values).T

decision_function


# * Визначаємо функцію втрати та обчислюємо оптимальність

# In[16]:


def loss_function(cipher_text: int, true_message: int) -> int:
    return 0 if decision_function[cipher_text][1] == true_message else 1

def average_losses() -> float:
    double_sum_variables = []
    for message in range(length_table):
        for cipher_text in range(length_table):
            var = probabilites_ciphertext_messages[message][cipher_text]*loss_function(cipher_text, message)
            double_sum_variables.append(var)
    return sum(double_sum_variables)

average_losses()


# Як ми можемо побачити, Середні втрати детерміністичної вирішуючої функції $\delta_{D}$ є досить близькими до одиниці для нашого варіанту, що є не дуже доброю ознакою для криптоаналітика. Перейдімо тепер до побудови стохастичної вирішуючої функції.
# 
# 5) Формуємо стохастичну вирішуючу функцію
#     * Знаходимо максимальну умовну ймовірність у рядку
#     * Дивимося чи не повторюється точно та ж ймовірність у рядку
#     * Як є повторення, то до цих комірок присвоєюмо значення $\dfrac{1}{s}$, де $s$ -- кількість повторень, інакше залишаємо як є -- 1 

# In[17]:


def build_stochastic_decision_function() -> list:
    stohastic_function = []
    for message in range(length_table):
        current_probability_set = condtitional_probabilities[message]
        max_probability = max(current_probability_set)
        max_probability_entrance = list(current_probability_set).count(max_probability)
        
        row = [1 / max_probability_entrance if current_probability_set[cipher_text] == max_probability else 0
        for cipher_text in range(length_table)]
        stohastic_function.append(row)
    return stohastic_function

stohastic_function_table = pd.DataFrame(build_stochastic_decision_function())


# In[18]:


stohastic_function_table


# Стосовно отриманих результатів можна сказати, що стохастична та баєсівська відрізняються нашому варіанті.
# 
# 6) Точно так само рахуємо втрати, але для стохастичної

# In[19]:


def stochastic_function_losses(true_text: int, cipher_text: int) -> float:
    variables_list = []
    for message in range(length_table):
        if message != true_text:
            variables_list.append(stohastic_function_table[message][cipher_text])
    return sum(variables_list)

def average_losses_stohastic() -> float:
    double_sum_variables = []
    for cipher_text in range(length_table):
        for message in range(length_table):
            var = probabilites_ciphertext_messages[message][cipher_text]*stochastic_function_losses(message, cipher_text)
            double_sum_variables.append(var)
    return sum(double_sum_variables)

average_losses_stohastic()


# In[19]:




