## LLaMA:  Open  and  Efficient  Foundation  Language  Models

∗ 
 ∗ 
 ∗

### Hugo  Touvron,  Thibaut  Lavril,  Gautier  Izacard,  Xavier  Martinet

### Marie-Anne  Lachaux,  Timothee  Lacroix,  Baptiste  Rozière,  Naman  Goyal

### Eric  Hambro,  Faisal  Azhar,  Aurelien  Rodriguez,  Armand  Joulin

∗ 
 ∗

### Edouard  Grave,  Guillaume  Lample

Meta  AI

Abstract
 performance,  a  smaller  one  trained  longer  will

ultimately  be  cheaper  at  inference.  For  instance,

We  introduce  LLaMA,  a  collection  of  founda

although  Hoffmann  et  al.  (2022)  recommends

tion  language  models  ranging  from  7B  to  65B

training  a   1 0B  model  on  200B  tokens,  we  find

3 parameters .  We  train  our  models  on  trillions

2 of  tokens,  and  show  that  it  is  possible  to  train
 that  the  performance  of  a  7B  model  continues  to

0 state-of-the-art  models  using  publicly  avail improve  even  after   1 T  tokens .

2  able  datasets  exclusively,  without  resorting

b to  proprietary  and  inaccessible  datasets .  In
 The  focus  of  this  work  is  to  train  a  series  of

e particular,  LLaMA- 1 3B  outperforms  GPT-3
 language  models  that  achieve  the  best  possible  per

F  ( 1 75B)  on  most  benchmarks,  and  LLaMA formance  at  various  inference  budgets,  by  training

7 65B  is  competitive  with  the  best  models,
 on  more  tokens  than  what  is  typically  used.  The

2 Chinchilla-70B  and  PaLM-540B .  We  release

1
 resulting  models ,  called  LLaMA ,  ranges  from  7B

] all  our  models  to  the  research  community .

to  65B  parameters  with  competitive  performance

# L

1  Introduction
 compared  to  the  best  existing  LLMs .  For  instance,

C. LLaMA- 1 3B  outperforms  GPT-3  on  most  bench

sc Large  Languages  Models  (LLMs)  trained  on  mas marks,  despite  being   1 0 ×   smaller.  We  believe  that

[ sive  corpora  of  texts  have  shown  their  ability  to  per this  model  will  help  democratize  the  access  and

1  form  new  tasks  from  textual  instructions  or  from  a
 study  of  LLMs,  since  it  can  be  run  on  a  single  GPU.

v few  examples  (Brown  et  al. ,  2020) .  These  few-shot
 At  the  higher-end  of  the  scale,  our  65B -parameter

1 properties  first  appeared  when  scaling  models  to  a
 model  is  also  competitive  with  the  best  large  lan

# 7

9 sufficient  size  (Kaplan  et  al. ,  2020) ,  resulting  in  a
 guage  models  such  as  Chinchilla  or  PaLM-540B .

3 line  of  work  that  focuses  on  further  scaling  these

1. models  (Chowdhery  et  al. ,  2022 ;  Rae  et  al. ,  202 1 ) .
 Unlike  Chinchilla,  PaLM,  or  GPT-3 ,  we  only

2 These  efforts  are  based  on  the  assumption  that
 use  publicly  available  data,  making  our  work  com

0 more  parameters  will  lead  to  better  performance.
 patible  with  open-sourcing,  while  most  existing

# 3

2 However,  recent  work  from  Hoffmann  et  al.  (2022)
 models  rely  on  data  which  is  either  not  publicly

:v shows  that,  for  a  given  compute  budget,  the  best
 available  or  undocumented  (e.g.  “B ooks –   2TB ”  or

i performances  are  not  achieved  by  the  largest  mod “Social  media  conversations”) .  There  exist  some

Xr els,  but  by  smaller  models  trained  on  more  data.
 exceptions,  notably  OPT  (Zhang  et  al. ,  2022) ,

a The  obj ective  of  the  scaling  laws  from  Hoff GPT-NeoX  (Black  et  al. ,  2022),  BLOOM  (Scao

mann  et  al .  (2022)  is  to  determine  how  to  best
 et  al. ,  2022)  and  GLM  (Zeng  et  al. ,  2022) ,  but  none

scale  the  dataset  and  model  sizes  for  a  particular
 that  are  competitive  with  PaLM-62B  or  Chinchilla.

training  compute  budget.  However,  this  obj ective

disregards  the  inference  budget,  which  becomes
 In  the  rest  of  this  paper,  we  present  an  overview

critical  when  serving  a  language  model  at  scale.
 of  the  modifications  we  made  to  the  transformer

In  this  context,  given  a  target  level  of  performance,
 architecture  (Vaswani  et  al . ,  20 1 7) ,  as  well  as  our

the  preferred  model  is  not  the  fastest  to  train  but  the
 training  method.  We  then  report  the  performance  of

fastest  at  inference,  and  although  it  may  be  cheaper
 our  models  and  compare  with  others  LLMs  on  a  set

to  train  a  large  model  to  reach  a  certain  level  of
 of  standard  benchmarks .  Finally,  we  expose  some

∗  of  the  biases  and  toxicity  encoded  in  our  models ,

Equal  contribution.  Correspondence:  { h touv ro n ,

t h i ba u t l av , g i za ca rd , eg rave , gl ampl e }@me ta . com
 using  some  of  the  most  recent  benchmarks  from

1
 h t t ps : / /g i t h u b . com/ fa ce book resea rch / l l ama
 the  responsible  AI  community.

2  Approach
 Dataset  Sampling  prop.  Epochs  Disk  size

Our  training  approach  is  similar  to  the  methods
 CommonCrawl  67 .0%   1 . 1 0  3 . 3  TB

described  in  previous  work  (Brown  et  al. ,  2020 ;
 C4   1 5 .0%   1 .06  7 8 3  GB

Chowdhery  et  al. ,  2022) ,  and  is  inspired  by  the
 Github  4 . 5 %  0. 64  3 28  GB

Chinchilla  scaling  laws  (Hoffmann  et  al. ,  2022) .
 Wikipedia  4 . 5 %  2.45  8 3  GB

We  train  large  transformers  on  a  large  quantity  of
 B ooks  4 . 5 %  2. 23  85  GB

textual  data  using  a  standard  optimizer.
 ArXiv  2. 5 %   1 .06  92  GB

2. 1  Pre-training  Data

StackExchange  2.0%   1 .03  7 8  GB

Our  training  dataset  is  a  mixture  of  several  sources ,
 Table   1 :  Pre-training  data.  Data  mixtures  used  for  pre

reported  in  Table  1 ,  that  cover  a  diverse  set  of  do training,  for  each  subset  we  list  the  sampling  propor

mains .  For  the  most  part,  we  reuse  data  sources
 tion,  number  of  epochs  performed  on  the  subset  when

training  on   1 .4T  tokens ,  and  disk  size.  The  pre-training

that  have  been  leveraged  to  train  other  LLMs,  with

runs  on   1 T  tokens  have  the  same  sampling  proportion.

the  restriction  of  only  using  data  that  is  publicly

available,  and  compatible  with  open  sourcing .  This

leads  to  the  following  mixture  of  data  and  the  per languages ,  which  use  either  the  Latin  or  Cyrillic

centage  they  represent  in  the  training  set : 
 scripts :   b g ,  c a ,  c s ,  d a ,  d e ,  e n ,  e s ,  f r ,  h r ,  h u ,  i t ,

English  CommonCrawl  [67 % ].  We  preprocess

five  CommonCrawl  dumps,  ranging  from  20 1 7

to  2020,  with  the  CCNet  pipeline  (Wenzek  et  al. ,

n l ,  p l ,  p t ,  r o ,  r u ,  s l ,  s r ,  s v ,  u k .  We  proces s  the

data  to  remove  hyperlinks,  comments  and  other

formatting  boilerplate.

2020) .  This  process  deduplicates  the  data  at  the

Gutenberg  and  Books3  [4.5 % ].  We  include

line  level,  performs  language  identification  with
 

two  book  corpora  in  our  training  dataset:  the  Guten

a  fastText  linear  classifier  to  remove  non-English

berg  Proj ect,  which  contains  books  that  are  in  the

pages  and  filters  low  quality  content  with  an  n 

public  domain,  and  the  Books3  section  of  TheP

gram  language  model.  In  addition,  we  trained  a

ile  (Gao  et  al . ,  2020) ,  a  publicly  available  dataset

linear  model  to  classify  pages  used  as  references

for  training  large  language  models .  We  perform

in  Wikipedia  v. s.   randomly  sampled  pages,  and

deduplication  at  the  book  level,  removing  books

discarded  pages  not  classified  as  references .

with  more  than  90%  content  overlap.

C4  [15 % ] .  During  exploratory  experiments,  we

-  ArXiv  [2.5 % ] .  We  process  arXiv  Latex  files

observed  that  using  diverse  pre processed  Com

to  add  scientific  data  to  our  dataset.  Following

monCrawl  datasets  improves  performance.  We  thus

Lewkowycz  et  al.  (2022) ,  we  removed  everything

included  the  publicly  available  C4  dataset  (Raffel

before  the  first  section,  as  well  as  the  bibliography.

et  al . ,  2020)  in  our  data.  The  preproces sing  of  C4

 We  also  removed  the  comments  from  the  .tex  files,

also  contains  deduplication  and  language  identifi

and  inline-expanded  definitions  and  macros  written

cation  steps :  the  main  difference  with  CCNet  is

 by  users  to  increase  consistency  acros s  papers .

the  quality  filtering,  which  mostly  relies  on  heuris

tics  such  as  presence  of  punctuation  marks  or  the

Stack  Exchange  [2 % ] .  We  include  a  dump  of

number  of  words  and  sentences  in  a  webpage.
 

Stack  Exchange,  a  website  of  high  quality  ques

Github  [4.5 % ] .  We  use  the  public  GitHub
 tions  and  answers  that  covers  a  diverse  set  of  do

dataset  available  on  Google  BigQuery.  We  only
 mains,  ranging  from  computer  science  to  chemistry.

kept  proj ects  that  are  distributed  under  the  Apache,
 We  kept  the  data  from  the  28  largest  websites ,  re

B SD  and  MIT  licenses .  Additionally,  we  filtered
 moved  the  HTML  tags  from  text  and  sorted  the

low  quality  files  with  heuristics  based  on  the  line
 answers  by  score  (from  highest  to  lowest) .

length  or  proportion  of  alphanumeric  characters,

 Tokenizer.  We  tokenize  the  data  with  the  byte

and  removed  boilerplate,  such  as  headers,  with  reg

 pair  encoding  (BPE)  algorithm  (Sennrich  et  al. ,

ular  expressions .  Finally,  we  deduplicate  the  result

20 1 5),  using  the  implementation  from  Sentence

ing  dataset  at  the  file  level,  with  exact  matches .

Piece  (Kudo  and  Richardson,  20 1 8) .  Notably,  we

Wikipedia  [4.5 % ] .  We  add  Wikipedia  dumps
 split  all  numbers  into  individual  digits,  and  fallback

from  the  June-August  2022  period,  covering  20
 to  bytes  to  decompose  unknown  UTF-8  characters .

params  dimension  n  heads  n  layers  learning  rate  batch  size  n  tokens

− 4

6 . 7B  4096  3 2  3 2  3 . 0e
 4M   1 . 0T

− 4

1 3 . 0B  5 1 20  40  40  3 . 0e
 4M   1 . 0T

− 4

3 2 . 5B  665 6  5 2  60  1 . 5 e
 4M   1 .4T

− 4

65 . 2B  8 1 92  64  80  1 . 5 e
 4M   1 .4T

Table  2:  Model  sizes,  architectures,  and  optimization  hyper-parameters.

Overall,  our  entire  training  dataset  contains
 2 . 2

LLa MA 7 B

roughly   1 .4T  tokens  after  tokenization.  For  most  of
 2 . 1

## LLa MA 1 3 B

our  training  data,  each  token  is  used  only  once  dur s 
 LLa MA 3 3 B

s 2 . 0

ing  training,  with  the  exception  of  the  Wikipedia
 lo  LLa MA 6 5 B

and  Books  domains,  over  which  we  perform  ap gn 1 . 9

# i

proximately  two  epochs .
 in 1 . 8

# ar

T 1 . 7

2.2  Architecture

1 . 6

Following  recent  work  on  large  language  models,

 1 . 5

our  network  is  based  on  the  transformer  architec 0   2 00   400   600   8 00   1 000   1 2 00   1 400

## ture  (Vaswani  et  al . ,  20 1 7) .  We  leverage  various
 B i l l i o n of to ke n s

improvements  that  were  subsequently  proposed,

and  used  in  different  models  such  as  PaLM .  Here
 Figure   1 :  Training  loss  over  train  tokens  for  the  7B,

 13B,  33B,  and  65  models.  LLaMA-33B  and  LLaMA

are  the  main  difference  with  the  original  architec

65B  were  trained  on   1 .4T  tokens .  The  smaller  models

ture,  and  where  we  were  found  the  inspiration  for

were  trained  on   1 .0T  tokens .  All  models  are  trained

this  change  (in  bracket) : 
 with  a  batch  size  of  4M  tokens .

Pre-normalization  [GPT3].  To  improve  the

training  stability,  we  normalize  the  input  of  each
 steps ,  and  vary  the  learning  rate  and  batch  size  with

transformer  sub-layer,  instead  of  normalizing  the
 the  size  of  the  model  (see  Table  2  for  details) .

output.  We  use  the  RMSNorm  normalizing  func

tion,  introduced  by  Zhang  and  Sennrich  (20 1 9) .
 2.4  Efficient  implementation

SwiGLU  activation  function  [PaLM].  We  re We  make  several  optimizations  to  improve  the  train

place  the  ReLU  non-linearity  by  the  SwiGLU  ac ing  speed  of  our  models .  First,  we  use  an  efficient

tivation  function,  introduced  by  Shazeer  (2020)  to
 implementation  of  the  causal  multi-head  attention

improve  the  performance.  We  use  a  dimension  of
 to  reduce  memory  usage  and  runtime.  This  imple

2
 4d  instead  of  4d  as  in  PaLM .
 mentation,  available  in  the  x fo rme r s  library,2
 is

3

inspired  by  Rabe  and  Staats  (202 1 )  and  uses  the

Rotary  Embeddings  [GPTNeo] .  We  remove  the
 backward  from  Dao  et  al.  (2022) .  This  is  achieved

absolute  positional  embeddings,  and  instead,  add
 b  not  storin  the  attention  wei hts  and  not  com

y g g

rotary  positional  embeddings  (RoPE) ,  introduced
 utin  the  ke / uer  scores  that  are  masked  due  to

p g y q y

by  Su  et  al .  (202 1 ) ,  at  each  layer  of  the  network.
 the  causal  nature  of  the  lan ua e  modelin  task.

g g g

The  details  of  the  hyper-parameters  for  our  dif 

To  further  improve  training  efficiency,  we  re

ferent  models  are  given  in  Table  2 .
 

duced  the  amount  of  activations  that  are  recom

2.3  O timizer
 puted  during  the  backward  pass  with  checkpoint

p

ing .  More  precisely,  we  save  the  activations  that

Our  models  are  trained  using  the  AdamW  opti

are  expensive  to  compute,  such  as  the  outputs  of

mizer  (Loshchilov  and  Hutter,  20 1 7) ,  with  the  fol 

linear  layers .  This  is  achieved  by  manually  imple

lowing  hyper-parameters :  β1  =  0 . 9 ,  β2  =  0 . 95 .
 mentin  the  backward  function  for  the  transformer

g

We  use  a  cosine  learning  rate  schedule,  such  that

layers ,  instead  of  relying  on  the  PyTorch  autograd.

the  final  learning  rate  is  equal  to   1 0%  of  the  maxi

To  fully  benefit  from  this  optimization,  we  need  to

mal  learning  rate.  We  use  a  weight  decay  of  0 . 1  and

gradient  clipping  of  1 . 0 .  We  use  2 ,  000  warmup
 2
https ://github.com/facebookresearch/xformers

BoolQ  PIQA  SIQA  HellaSwag  WinoGrande  ARC-e  ARC-c  OBQA

GPT- 3   1 75B  60 . 5  8 1 . 0  -  7 8 . 9  70 . 2  6 8 . 8  5 1 .4  57 . 6

Gopher  280B  79 . 3  8 1 . 8  5 0 . 6  79 . 2  70 . 1  -  -  -

Chinchilla  70B  8 3 . 7  8 1 . 8  5 1 . 3  80 . 8  74 . 9  -  -  -

PaLM  62B  84 . 8  80 . 5  -  79 . 7  77 . 0  75 . 2  5 2 . 5  50 .4

PaLM-cont  62B  8 3 . 9  8 1 .4  -  80 . 6  77 . 0  -  -  -

PaLM  540B  88.0  82 . 3  -  8 3 .4  8 1 . 1  76 . 6  5 3 . 0  5 3 .4

LLaMA

7B  7 6 . 5  7 9 . 8  4 8 . 9  7 6 . 1  70 . 1  7 2 . 8  47 . 6  5 7 . 2

1 3 B  7 8 . 1  80 . 1  5 0 . 4  7 9 . 2  7 3 . 0  74 . 8  5 2 . 7  5 6 . 4

3 3 B  8 3 . 1  8 2 . 3  5 0 .4  8 2 . 8  7 6 . 0  80.0  57.8  5 8 . 6

65B  8 5 . 3  82.8  52.3  84.2  77 . 0  7 8 . 9  5 6 . 0  60.2

Table  3 :  Zero-shot  performance  on  Common  Sense  Reasoning  tasks.

reduce  the  memory  usage  of  the  model  by  using
 We  evaluate  LLaMA  on  free-form  generation

model  and  sequence  parallelism,  as  described  by
 tasks  and  multiple  choice  tasks .  In  the  multiple

Korthikanti  et  al.  (2022) .  Moreover,  we  also  over choice  tasks ,  the  obj ective  is  to  select  the  most

lap  the  computation  of  activations  and  the  commu appropriate  completion  among  a  set  of  given  op

nication  between  GPUs  over  the  network  (due  to
 tions,  based  on  a  provided  context.  We  select  the

a l l _ re d u ce  operations)  as  much  as  possible.
 completion  with  the  highest  likelihood  given  the

When  training  a  65B -parameter  model,  our  code
 provided  context.  We  follow  Gao  et  al.  (202 1 )

processes  around  3 80  tokens/sec/GPU  on  2048
 and  use  the  likelihood  normalized  by  the  number

A 1 00  GPU  with  80GB  of  RAM.  This  means  that
 of  characters  in  the  completion,  except  for  certain

training  over  our  dataset  containing   1 .4T  tokens
 datasets  (OpenBookQA,  BoolQ) ,  for  which  we  fol

takes  approximately  2 1  days .
 low  Brown  et  al.  (2020) ,  and  select  a  completion

based  on  the  likelihood  normalized  by  the  likeli

3  Main  results
 hood  of  the  completion  given  “Answer: ”  as  context:

Following  previous  work  (Brown  et  al. ,  2020) ,  we
 P ( complet ion | cont ext ) /P ( complet ion | “Answer : ” ) .

consider  zero- shot  and  few- shot  tasks ,  and  report

results  on  a  total  of  20  benchmarks :
 0- shot   1 - shot  5 - shot  64- shot

•  Zero-shot.  We  provide  a  textual  description
 GPT-3   1 75B   1 4 . 6  23 .0  -  29 . 9

of  the  task  and  a  test  example.  The  model
 Gopher  280B   1 0 . 1  -  24 . 5  28 . 2

either  provides  an  answer  using  open-ended
 Chinchilla  70B   1 6 . 6  -  3 1 . 5  3 5 . 5

generation,  or  ranks  the  proposed  answers .
 -

8 B  8 . 4   1 0 . 6     1 4 . 6

•  Few-shot.  We  provide  a  few  examples  of  the
 PaLM
 62B   1 8 . 1  26 . 5  -  27 . 6

task  (between   1  and  64)  and  a  test  example .
 540B  2 1 . 2  29 . 3  -  3 9 . 6

The  model  takes  this  text  as  input  and  gener 7B   1 6 . 8   1 8 .7  22 . 0  26 . 1

ates  the  answer  or  ranks  different  options .
 1 3B  20 . 1  23 .4  28 . 1  3 1 . 9

LLaMA

We  compare  LLaMA  with  other  foundation  mod 3 3B  24.9  28 . 3  32.9  36.0

els ,  namely  the  non-publicly  available  language
 65B  23 . 8  3 1 .0  35.0  39.9

models  GPT-3  (Brown  et  al. ,  2020) ,  Gopher  (Rae

et  al. ,  202 1 ) ,  Chinchilla  (Hoffmann  et  al. ,  2022)
 Table  4 :  NaturalQuestions.  Exact  match  performance.

and  PaLM  (Chowdhery  et  al. ,  2022) ,  as  well  as

the  open- sourced  OPT  models  (Zhang  et  al. ,  2022) ,

GPT-J  (Wang  and  Komatsuzaki,  202 1 ),  and  GPT

Neo  (B lack  et  al . ,  2022) .  In  S ection  4,  we  also

3.1  Common  Sense  Reasoning

briefly  compare  LLaMA  with  instruction-tuned
 We  consider  eight  standard  common  sense  rea

models  such  as  OPT-IML  (Iyer  et  al. ,  2022)  and
 soning  benchmarks :  B oolQ  (Clark  et  al. ,  20 1 9) ,

Flan-PaLM  (Chung  et  al. ,  2022) .
 PIQA  (Bisk  et  al. ,  2020) ,  SIQA  (S ap  et  al. ,  20 1 9) ,

HellaSwag  (Zellers  et  al. ,  20 1 9),  WinoGrande  (Sak RACE-middle  RACE-high

aguchi  et  al. ,  202 1 ) ,  ARC  easy  and  challenge  (Clark

GPT-3   1 75B  5 8 .4  45 . 5

et  al. ,  20 1 8)  and  OpenB ookQA  (Mihaylov  et  al. ,

20 1 8) .  These  datasets  include  Cloze  and  Winograd
 8B  57 . 9  42 . 3

style  tasks ,  as  well  as  multiple  choice  question  an PaLM
 62B  64 . 3  47 . 5

swering .  We  evaluate  in  the  zero- shot  setting  as
 540B  68. 1  49 . 1

done  in  the  language  modeling  community.

In  Table  3 ,  we  compare  with  existing  models

of  various  sizes  and  report  numbers  from  the  cor LLaMA

responding  papers .  First,  LLaMA-65B  outper

forms  Chinchilla-70B  on  all  reported  benchmarks

7B  6 1 . 1  46 . 9

1 3 B  6 1 . 6  47 . 2

3 3 B  64 . 1  4 8 . 3

65B  67 . 9  5 1 .6

but  BoolQ.  Similarly,  this  model  surpasses  PaLM Table  6:  Reading  Comprehension.  Zero-shot  accu

540B  everywhere  but  on  BoolQ  and  WinoGrande.
 racy.

LLaMA- 1 3B  model  also  outperforms  GPT-3  on

most  benchmarks  despite  being   1 0 ×   smaller.

school  Chinese  students .  We  follow  the  evaluation

3.2  Closed-book  Question  Answering
 setup  from  Brown  et  al.  (2020)  and  report  results

in  Table  6.  On  these  benchmarks,  LLaMA-65B  is

We  compare  LLaMA  to  existing  large  language

- competitive  with  PaLM-540B ,  and,  LLaMA- 1 3B

models  on  two  closed book  question  answering

outperforms  GPT-3  by  a  few  percents .

benchmarks :  Natural  Questions  (Kwiatkowski

et  al. ,  20 1 9)  and  TriviaQA  (Joshi  et  al. ,  20 1 7) .  For
 3.4  Mathematical  reasonin

g

both  benchmarks,  we  report  exact  match  perfor

 We  evaluate  our  models  on  two  mathematical  rea

mance  in  a  closed  book  setting,  i.e. ,  where  the  mod

els  do  not  have  access  to  documents  that  contain
 soning  benchmarks :  MATH  (Hendrycks  et  al. ,

evidence  to  answer  the  question.  In  Table  4,  we
 202 1 )  and  GSM8k  (Cobbe  et  al. ,  202 1 ) .  MATH

re ort   erformance  on  NaturalQuestions,  and  in  Ta is  a  dataset  of   1 2K  middle  school  and  high  school

p p

ble  5 ,  we  report  on  TriviaQA.  On  both  benchmarks,
 mathematics  problems  written  in  LaTeX.  GSM8k

LLaMA-65B  achieve  state-of-the-arts   erformance
 is  a  set  of  middle  school  mathematical  problems .

p

in  the  zero-shot  and  few-shot  settin s .  More  im In  Table  7 ,  we  compare  with  PaLM  and  Min

g

ortantl ,  the  LLaMA- 1 3B  is  also  com etitive  on
 erva  (Lewkowycz  et  al. ,  2022) .  Minerva  is  a  series

p y p

these  benchmarks  with  GPT-3  and  Chinchilla,  de of  PaLM  models  finetuned  on  3 8 .5B  tokens  ex

s ite  bein  5 - 1 0 ×   smaller.  This  model  runs  on  a
 tracted  from  ArXiv  and  Math  Web  Pages,  while

p g

neither  PaLM  or  LLaMA  are  finetuned  on  mathe

single  V 1 00  GPU  during  inference.

matical  data.  The  numbers  for  PaLM  and  Minerva

- - - - are  taken  from  Lewkowycz  et  al.  (2022) ,  and  we

0 shot   1 shot  5 shot  64 shot

compare  with  and  without  ma j 1 @k.  ma j 1 @k  de

Gopher  280B  43 . 5  -  57 .0  57 . 2
 notes  evaluations  where  we  generate  k  samples  for

Chinchilla  70B  55 .4  -  64 . 1  64 . 6
 each  problem  and  perform  a  maj ority  voting  (Wang

7B  50.0  5 3 .4  5 6 . 3  57 . 6
 et  al. ,  2022) .  On  GSM8k,  we  observe  that  LLaMA

1 3B  5 6 . 6  60 . 5  63 . 1  64 .0
 65B  outperforms  Minerva-62B ,  although  it  has  not

LLaMA
 3 3B  65 . 1  67 . 9  69 . 9  70.4
 been  fine-tuned  on  mathematical  data.

65B  68.2  71 .6  72.6  73.0

3.5  Code  generation

Table  5 :  TriviaQA.  Zero-shot  and  few-shot  exact
 We  evaluate  the  ability  of  our  models  to  write

match  performance  on  the  filtered  dev  set.
 code  from  a  natural  language  description  on  two

benchmarks :  HumanEval  (Chen  et  al. ,  202 1 )  and

3.3  Reading  Comprehension

MBPP  (Austin  et  al . ,  202 1 ) .  For  both  tasks ,  the

model  receives  a  description  of  the  program  in  a

We  evaluate  our  models  on  the  RACE  reading  com few  sentences,  as  well  as  a  few  input-output  ex

prehension  benchmark  (Lai  et  al. ,  20 1 7) .  This
 amples .  In  HumanEval,  it  also  receives  a  function

dataset  was  collected  from  English  reading  com signature,  and  the  prompt  is  formatted  as  natural

prehension  exams  designed  for  middle  and  high
 code  with  the  textual  description  and  tests  in  a

MATH  +maj 1 @k  GSM8k  +maj 1 @k
 Params  HumanEval  MBPP

- - pass @   @ 1   @ 1 00   @ 1   @ 80

8 B   1 . 5    4 . 1

PaLM
 62B  4 .4  -  3 3 . 0  -
 LaMDA   1 37B   1 4 . 0  47 . 3   1 4 . 8  62 .4

540B  8 . 8  -  5 6 . 5  -
 PaLM  8B  3 . 6 ∗  1 8 . 7 ∗  5 . 0∗  3 5 . 7 ∗

8B   1 4 . 1  25 . 4   1 6 . 2  2 8 . 4

Minerva
 62B  27 . 6  43 .4  5 2 .4  68 . 5

540B  33.6  50.3  68.5  78.5

PaLM  62B   1 5 . 9  46 . 3 ∗  2 1 .4  63 . 2∗

PaLM-cont  62B  23 .7  -  3 1 . 2  -

PaLM  540B  26.2  76 . 2  3 6 . 8  75 .0

7B   1 0 . 5  3 6 . 5   1 7 . 7  5 6 . 2

7B  2 . 9  6 . 9   1 1 . 0   1 8 . 1

1 3 B   1 5 . 8  5 2 . 5  22 . 0  64 . 0

1 3B  3 . 9  8 . 8   1 7 . 8  29 . 3
 LLaMA

LLaMA
 3 3B  2 1 .7  70 .7  30 . 2  7 3 .4

3 3 B  7 . 1   1 5 . 2  3 5 . 6  5 3 . 1

65B  23 . 7  79.3  37.7  76.8

65B   1 0 . 6  20 . 5  5 0 . 9  69 . 7

 Table  8 :  Model  performance  for  code  generation.

Table  7 :  Model  performance  on  quantitative  reason

We  report  the  pass @  score  on  HumanEval  and  MBPP.

ing  datasets.  For  maj ority  voting,  we  use  the  same
 -

= HumanEval  generations  are  done  in  zero shot  and

setup  as  Minerva,  with  k   2 56  samples  for  MATH

= = MBBP  with  3 -shot  prompts  similar  to  Austin  et  al.

and  k   1 00  for  GSM8k  (Minerva  540B  uses  k   64
 ∗

= - (202 1 ) .  The  values  marked  with  are  read  from  figures

for  MATH  and  and  k   40  for  GSM8k) .  LLaMA 65B

in  Chowdhery  et  al.  (2022) .

outperforms  Minerva  62B  on  GSM8k,  although  it  has

not  been  fine-tuned  on  mathematical  data.

3.6  Massive  Multitask  Language

docstring .  The  model  needs  to  generate  a  Python

Understanding

program  that  fits  the  description  and  satisfies  the
 The  massive  multitask  language  understanding

test  cases .  In  Table  8 ,  we  compare  the  pass @ 1
 benchmark,  or  MMLU,  introduced  by  Hendrycks

scores  of  our  models  with  existing  language  mod et  al.  (2020)  consists  of  multiple  choice  questions

els  that  have  not  been  finetuned  on  code,  namely
 covering  various  domains  of  knowledge,  includ

PaLM  and  LaMDA  (Thoppilan  et  al. ,  2022) .  PaLM
 ing  humanities,  STEM  and  social  sciences .  We

and  LLaMA  were  trained  on  datasets  that  contain
 evaluate  our  models  in  the  5 - shot  setting,  using  the

a  similar  number  of  code  tokens .
 examples  provided  by  the  benchmark,  and  report

As  show  in  Table  8 ,  for  a  similar  number
 results  in  Table  9 .  On  this  benchmark,  we  observe

of  parameters,  LLaMA  outperforms  other  gen that  the  LLaMA-65B  is  behind  both  Chinchilla

eral  models  such  as  LaMDA  and  PaLM,  which
 70B  and  PaLM-540B  by  a  few  percent  in  average,

are  not  trained  or  finetuned  specifically  for  code.
 and  across  most  domains .  A  potential  explanation

LLaMA  with   1 3B  parameters  and  more  outper is  that  we  have  used  a  limited  amount  of  books

forms  LaMDA   1 37B  on  both  HumanEval  and
 and  academic  papers  in  our  pre-training  data,  i.e. ,

MBPP.  LLaMA  65B  also  outperforms  PaLM  62B ,
 ArXiv,  Gutenberg  and  Books3 ,  that  sums  up  to  only

even  when  it  is  trained  longer.  The  pass @ 1  results
 1 77GB ,  while  these  models  were  trained  on  up  to

reported  in  this  table  were  obtained  by  sampling
 2TB  of  books .  This  large  quantity  of  books  used

with  temperature  0. 1 .  The  pass @ 1 00  and  pass @ 80
 by  Gopher,  Chinchilla  and  PaLM  may  also  explain

metrics  were  obtained  with  temperature  0. 8 .  We
 why  Gopher  outperforms  GPT-3  on  this  benchmark,

use  the  same  method  as  Chen  et  al.  (202 1 )  to  obtain
 while  it  is  comparable  on  other  benchmarks .

unbiased  estimates  of  the  pass @ k.

It  is  possible  to  improve  the  performance  on  code

3.7  Evolution  of  performance  during  training

by  finetuning  on  code-specific  tokens .  For  instance,
 During  training,  we  tracked  the  performance  of  our

PaLM-Coder  (Chowdhery  et  al. ,  2022)  increases
 models  on  a  few  question  answering  and  common

the  pass @ 1  score  of  PaLM  on  HumanEval  from
 sense  benchmarks,  and  report  them  in  Figure  2.

26.2%  for  PaLM  to  36% .  Other  models  trained
 On  most  benchmarks,  the  performance  improves

specifically  for  code  also  perform  better  than  gen steadily,  and  correlates  with  the  training  perplexity

eral  models  on  these  tasks  (Chen  et  al. ,  202 1 ;  Ni of  the  model  (see  Figure  1 ) .  The  exceptions  are

jkamp  et  al. ,  2022 ;  Fried  et  al. ,  2022) .  Finetuning
 SIQA  and  WinoGrande.  Most  notably,  on  SIQA,

on  code  tokens  is  beyond  the  scope  of  this  paper.
 we  observe  a  lot  of  variance  in  performance,

Humanities  STEM  Social  Sciences  Other  Average

GPT-NeoX  20B  29 . 8  34 . 9  3 3 .7  37 .7  3 3 . 6

GPT- 3   1 75B  40 . 8  3 6 .7  50 .4  48 . 8  43 . 9

Gopher  280B  5 6 . 2  47 .4  7 1 . 9  66 . 1  60 . 0

Chinchilla  70B  63 . 6  54 . 9  79 . 3  73.9  67 . 5

8B  25 . 6  23 . 8  24 . 1  27 . 8  25 . 4

PaLM
 62B  5 9 . 5  4 1 . 9  62 . 7  5 5 . 8  5 3 . 7

540B  77.0  55.6  8 1 .0  69 . 6  69.3

LLaMA

7B  3 4 . 0  3 0 . 5  3 8 . 3  3 8 . 1  3 5 . 1

1 3 B  45 . 0  3 5 . 8  5 3 . 8  5 3 . 3  46 . 9

3 3 B  5 5 . 8  46 . 0  66 . 7  63 .4  57 . 8

65B  6 1 . 8  5 1 . 7  72 . 9  67 .4  63 .4

Table  9 :  Massive  Multitask  Language  Understanding  (MMLU).  Five-shot  accuracy.

that  may  indicate  that  this  benchmark  is  not
 In  Table  1 0,  we  report  the  results  of  our  instruct

reliable.  On  WinoGrande,  the  performance  does
 model  LLaMA-I  on  MMLU  and  compare  with  ex

not  correlate  as  well  with  training  perplexity :
 isting  instruction  finetuned  models  of  moderate

the  LLaMA-3 3B  and  LLaMA-65B  have  similar
 sizes,  namely,  OPT-IML  (Iyer  et  al. ,  2022)  and  the

performance  during  the  training .
 Flan-PaLM  series  (Chung  et  al. ,  2022) .  All  the  re

ported  numbers  are  from  the  corresponding  papers .

4  Instruction  Finetuning
 Despite  the  simplicity  of  the  instruction  finetuning

approach  used  here,  we  reach  68 .9%  on  MMLU.

In  this  section,  we  show  that  briefly  finetuning  on
 LLaMA-I   65B  out erforms  on  MMLU  existin

( ) p g

instructions  data  rapidly  leads  to  improvements
 instruction  finetuned  models  of  moderate  sizes,  but

on  MMLU.  Although  the  non-finetuned  version
 are  still  far  from  the  state-of-the-art,  that  is  77 .4

of  LLaMA-65B  is  already  able  to  follow  basic  in - -

for  GPT  code dav i n c i 00 2  on  MMLU  (numbers

structions ,  we  observe  that  a  very  small  amount  of
 taken  from  I er  et  al .  (2022)) .  The  details  of  the

y

finetuning  improves  the  performance  on  MMLU,
 erformance  on  MMLU  on  the  57  tasks  can  be

p

and  further  improves  the  ability  of  the  model  to
 found  in  Table  1 6  of  the  a endix.

pp

follow  instructions .  Since  this  is  not  the  focus  of

this  paper,  we  only  conducted  a  single  experiment

following  the  same  protocol  as  Chung  et  al.  (2022)
 5  Bias,  Toxicity  and  Misinformation

to  train  an  instruct  model,  LLaMA-I.

Large  language  models  have  been  showed  to  re

OPT  30B  26 . 1 
 produce  and  amplify  biases  that  are  existing  in

GLM   1 20B  44 . 8
 the  training  data  (Sheng  et  al . ,  20 1 9 ;  Kurita  et  al . ,

PaLM  62B  55 . 1 
 20 1 9) ,  and  to  generate  toxic  or  offensive  con

PaLM-cont  62B  62 . 8
 tent  (Gehman  et  al. ,  2020) .  As  our  training  dataset

Chinchilla  70B  67 . 5
 contains  a  large  proportion  of  data  from  the  Web,

LLaMA  65B  63 .4
 we  believe  that  it  is  crucial  to  determine  the  po

OPT-IML-Max  30B  43 .2
 tential  for  our  models  to  generate  such  content.

Flan-T5-XXL   1 1 B  55 . 1 
 To  understand  the  potential  harm  of  LLaMA-65B ,

Flan-PaLM  62B  59 .6
 we  evaluate  on  different  benchmarks  that  measure

Flan-PaLM-cont  62B  66 . 1 
 toxic  content  production  and  stereotypes  detection.

LLaMA-I  65B  68.9
 While  we  have  selected  some  of  the  standard  bench

marks  that  are  used  by  the  language  model  com

Table   1 0:  Instruction  finetuning –   MMLU  (5-shot).
 munity  to  indicate  some  of  the  issues  with  these

Comparison  of  models  of  moderate  size  with  and  with models ,  these  evaluations  are  not  sufficient  to  fully

out  instruction  finetuning  on  MMLU.
 understand  the  risks  associated  with  these  models .

Tri v i a QA 
 H e l l a Swa g 
 N a t u ra l Q u e st i o n s

8 5

7 0 
 3 5

8 0

6 0 
 3 0

7 5

cy 
 2 5

ar 5 0 
 7 0

u 2 0

cc 6 5

A 4 0 
 1 5

3 0

6 0 
 1 0

5 5 
 5

2 0 
 5 0 
 0

0   2 5 0   5 0 0   7 5 0   1 0 0 0   1 2 5 0   1 5 0 0 
 0   2 5 0   5 0 0   7 5 0   1 0 0 0   1 2 5 0   1 5 0 0 
 0   2 5 0   5 0 0   7 5 0   1 0 0 0   1 2 5 0   1 5 0 0

S I QA 
 W i n o G ra n d e 
 P I QA

8 0

5 2 
 8 2 . 5

5 0 
 7 5 
 8 0 . 0

cy 4
 8 
 7 0 
 7 7 . 5

aru 6 5 
 7 5 . 0

cc 4 6 
 LLa MA 7 B

A 7 2 . 5

4 4 
 6 0 
 L La M A  1 3 B

7 0 . 0 
 LLa MA 3 3 B

4 2 
 5 5 
 6 7 . 5 
 L La M A 6 5 B

C h i n c h i l l a

4 0 
 5 0 
 6 5 . 0

0   2 5 0   5 0 0   7 5 0   1 0 0 0   1 2 5 0   1 5 0 0 
 0   2 5 0   5 0 0   7 5 0   1 0 0 0   1 2 5 0   1 5 0 0 
 0   2 5 0   5 0 0   7 5 0   1 0 0 0   1 2 5 0   1 5 0 0

B i l l i o n of to ke n s 
 B i l l i o n of to ke n s 
 B i l l i o n of to ke n s

Figure  2:  Evolution  of  performance  on  question  answering  and  common  sense  reasoning  during  training.

5.1  RealToxicityPrompts
 B asic  Respectful

Language  models  can  generate  toxic  language,  e. g . ,
 7B  0 . 1 06  0 . 08 1

insults ,  hate  speech  or  threats .  There  is  a  very  large
 1 3B  0 . 1 04  0 . 095

range  of  toxic  content  that  a  model  can  generate,
 LLaMA
 3 3B  0. 1 07  0.087

making  a  thorough  evaluation  challenging .  Several
 65B  0. 1 28  0. 1 4 1

recent  work  (Zhang  et  al. ,  2022 ;  Hoffmann  et  al. ,

2022)  have  considered  the  RealToxicityPrompts
 Table  1 1 :  RealToxicityPrompts.  We  run  a  greedy  de

benchmark  (Gehman  et  al. ,  2020)  as  an  indicator
 coder  on  the   1 00k  prompts  from  this  benchmark.  The

of  how  toxic  is  their  model.  RealToxicityPrompts
 “respectful”  versions  are  prompts  starting  with  “Com

consists  of  about  1 00k   rom ts  that  the  model  must
 plete  the  following  sentence  in  a  polite,  respectful,  and

p p ” “ ”

unbiased  manner: ,  and   B asic  is  without  it.  Scores

complete ;  then  a  toxicity  score  is  automatically

3
 were  obtained  using  the  PerplexityAPI,  with  higher

evaluated  by  making  a  request  to  PerspectiveAPI  .
 score  indicatin  more  toxic   enerations .

g g

We  do  not  have  control  over  the  pipeline  used  by

the  third-party  PerspectiveAPI,  making  comparison

with  previous  models  difficult.

For  each  of  the  1 00k  prompts,  we  greedily  gen

erate  with  our  models,  and  measure  their  toxic

ity  score.  The  score  per  prompt  ranges  from  0

- with  the  size  of  the  model,  especially  for  Respect

(non toxic)  to   1  (toxic) .  In  Table  1 1 ,  we  report  our

 ful  prompts .  This  was  also  observed  in  previous

averaged  score  on  basic  and  respectful  prompt  cat

work  (Zhang  et  al. ,  2022) ,  with  the  notable  excep

egories  of  RealToxicityPrompts .  These  scores  are

“ ”  tion  of  Hoffmann  et  al.  (2022)  where  they  do  not

comparable  with  what  we  observe  in  the  litera

 see  a  difference  between  Chinchilla  and  Gopher,

ture  (e. g . ,  0.087  for  Chinchilla)  but  the  method

despite  different  sizes .  This  could  be  explained  by

ologies  differ  between  these  work  and  ours  (in

the  fact  that  the  larger  model,  Gopher,  has  worse

terms  of  sampling  strategy,  number  of  prompts  and

performance  than  Chinchilla,  suggesting  that  the

time  of  API) .  We  observe  that  toxicity  increases

relation  between  toxicity  and  model  size  may  only

3
 h t t ps : / / pe rs pe c t i vea p i . com/
 apply  within  a  model  family.

LLaMA  GPT3  OPT
 the  context  of  the  sentence.  The  goal  is  to  reveal

if  societal  biases  associated  with  occupations

Gender  70 . 6  62.6  65 .7

have  been  captured  by  the  model.  For  example,

Religion  79 . 0  7 3 . 3  68.6
 “

a  sentence  in  the  WinoGender  dataset  is   The

Race/Color  57.0  64 .7  68 . 6

nurse  notified  the  patient  that  his  shift  would  be

S exual  orientation  8 1 . 0  76.2  7 8 . 6
 ” ‘ ’

ending  in  an  hour. ,  which  is  followed  by  H i s

Age  70 . 1   64.4  67 . 8

re fe r s  t o .  We  then  compare  the  perplexity  of  the

Nationality  64 . 2  6 1 .6  62 . 9
 

continuations  t h e  n u r s e  and  t h e  pa t i e n t  to  per

Disability  66.7  76 . 7  76 . 7
 -

form  co reference  resolution  with  the  model.  We

Physical  appearance  77 . 8  74.6  76 . 2

evaluate  the  performance  when  using  3  pronouns :

Socioeconomic  status  71 .5  73 . 8  76 . 2
 “ ” “ ” “ 

her/her/she ,   his/him/he  and   their/them/some

”

Average  66.6  67 . 2  69 . 5
 one  (the  different  choices  corresponding  to  the

grammatical  function  of  the  pronoun.

Table   1 2 :  CrowS-Pairs.  We  compare  the  level  of  bi In  Table  1 3 ,  we  report  the  co-reference  scores

ases  contained  in  LLaMA-65B  with  OPT- 1 75B  and

for  the  three  different  pronouns  contained  in  the

GPT3 - 1 75B .  Higher  score  indicates  higher  bias .

dataset.  We  observe  that  our  model  is  significantly

better  at  performing  co-reference  resolution  for

“ ”

the   their/them/someone  pronouns  than  for  the

5.2  CrowS-Pairs
 “her/her/she”  and  “his/him/he”  pronouns .  A  simi

 lar  observation  was  made  in  previous  work  (Rae

We  evaluate  the  biases  in  our  model  on  the  CrowS

et  al . ,  202 1 ;  Hoffmann  et  al . ,  2022) ,  and  is  likely

Pairs  (Nangia  et  al . ,  2020) .  This  dataset  allows  to

indicative  of  gender  bias .  Indeed,  in  the  case  of  the

measure  biases  in  9  categories :  gender,  religion,
 “ ” “ ”

 her/her/she  and   his/him/he  pronouns,  the  model

race/color,  sexual  orientation,  age,  nationality,  dis

 is  probably  using  the  maj ority  gender  of  the  occu

ability,  physical  appearance  and  socioeconomic  sta

pation  to  perform  co-reference  resolution,  instead

tus .  Each  example  is  composed  of  a  stereotype  and

-  of  using  the  evidence  of  the  sentence.

an  anti stereotype,  we  measure  the  model  prefer

ence  for  the  stereotypical  sentence  using  the  per To  further  investigate  this  hypothesis ,  we  look

“ ” “ ”

plexity  of  both  sentences  in  a  zero- shot  setting .
 at  the  set  of   gotcha  cases  for  the   her/her/she

“ ”

Higher  scores  thus  indicate  higher  bias .  We  com and   his/him/he  pronouns  in  the  WinoGender

are  with  GPT-3  and  OPT- 1 75B  in  Table  1 2.
 dataset.  Theses  cases  correspond  to  sentences  in

p

which  the  pronoun  does  not  match  the  maj ority

LLaMA  compares  slightly  favorably  to  both

models  on  avera e.  Our  model  is   articularl  bi gender  of  the  occupation,  and  the  occupation  is

g p y

the  correct  answer.  In  Table  1 3 ,  we  observe  that

ased  in  the  religion  category  (+ 1 0%  compared  to

-  our  model,  LLaMA-65B ,  makes  more  errors  on  the

OPT 1 75B),  followed  by  age  and  gender.  We  ex

ect  these  biases  to  come  from  CommonCrawl  de gotcha  examples,  clearly  showing  that  it  capture

p

societal  biases  related  to  gender  and  occupation.

spite  multiple  filtering  steps .
 “ ”

The  drop  of  performance  exists  for   her/her/she

“ ”

5.3  WinoGender
 and   his/him/he  pronouns,  which  is  indicative  of

To  further  investigate  the  biases  of  our  model  on

biases  regardles s  of  gender.

the  gender  category,  we  look  at  the  WinoGender

- 5.4  TruthfulQA

benchmark  (Rudinger  et  al. ,  20 1 8) ,  a  co reference

resolution  dataset.  WinoGender  is  made  of  Wino TruthfulQA  (Lin  et  al. ,  202 1 )  aims  to  measure  the

grad  schema,  and  biases  are  evaluated  by  determin truthfulnes s  of  a  model,  i . e. ,  its  ability  to  identify

ing  if  a  model  co-reference  resolution  performance
 when  a  claim  is  true.  Lin  et  al.  (202 1 )  consider

is  impacted  by  the  gender  of  the  pronoun.
 the  definition  of  “true”  in  the  sense  of  “literal  truth

More  precisely,  each  sentence  has  three  men about  the  real  world” ,  and  not  claims  that  are  only

“ ” “ ”

tions :  an   occupation ,  a   participant ,  and  a
 true  in  the  context  of  a  belief  system  or  tradition.

“pronoun”  where  the  pronoun  is  co-referencing
 This  benchmark  can  evaluate  the  risks  of  a  model

either  the  occupation  or  participant.  We  prompt
 to  generate  misinformation  or  false  claims .  The

the  model  to  determine  the  co-reference  relation
 questions  are  written  in  diverse  style,  cover  3 8  cat

and  measure  if  it  does  so  correctly  according  to
 egories  and  are  designed  to  be  adversarial.

7B   1 3B  3 3B  65B
 where  we  set  the  Power  Usage  Effectiveness  (PUE)

at  1 . 1 .  The  resulting  carbon  emission  depends  on

All  66 . 0  64 . 7  69 . 0  77 . 5
 

the  location  of  the  data  center  used  to  train  the  net

her/her/she  65 .0  66 .7  66 .7  7 8 . 8
 work.  For  instance,  BLOOM  uses  a  grid  that  emits

his/him/he  60. 8  62. 5  62. 1  72. 1 
 0.057  kg  CO2 eq/KWh  leading  to  27  tCO2 eq  and

their/them/someone  72. 1  65 .0  7 8 . 3  8 1 .7
 OPT  a  grid  that  emits  0.23 1  kg  CO2 eq/KWh,  lead

her/her/she  (gotcha)  64 . 2  65 . 8  6 1 .7  75 . 0

his/him/he  (gotcha)  5 5 . 0  5 5 . 8  5 5 . 8  63 . 3

ing  to  82  tCO2 eq.  In  this  study,  we  are  interested  in

comparing  the  cost  in  carbon  emission  of  training

of  these  models  if  they  were  trained  in  the  same

Table   1 3 :  WinoGender.  Co-reference  resolution  ac data  center.  Hence,  we  do  not  take  the  location

curacy  for  the  LLaMA  models,  for  different  pronouns
 of  data  center  in  consideration,  and  use,  instead,

(“her/her/she”  and  “his/him/he”) .  We  observe  that  our
 the  US  national  average  carbon  intensity  factor  of

models  obtain  better  performance  on  “their/them/some 0. 3 85  kg  CO2 eq/KWh.  This  leads  to  the  following

one’  pronouns  than  on  “her/her/she”  and  “his/him/he’ ,
 formula  for  the  tons  of  carbon  emissions :

which  is  likely  indicative  of  biases .

tCO2 eq  =  MWh  ×   0 . 385 .

Truthful  Truthful*Inf

We  apply  the  same  formula  to  OPT  and  BLOOM

1 . 3 B  0 . 3 1  0 . 1 9

for  fair  comparison.  For  OPT,  we  assume  training

GPT- 3
 6B  0 . 22  0 . 1 9
 - 4

required  34  days  on  992  A 1 00 80B  (see  their  logs ) .

1 75B  0 . 28  0 . 25
 -

Finally,  we  estimate  that  we  used  2048  A 1 00 80GB

7B  0. 3 3  0. 29
 for  a  period  of  approximately  5  months  to  develop

LLaMA

1 3B  0.47  0.4 1 
 our  models .  This  means  that  developing  these  mod

3 3B  0. 52  0.48
 els  would  have  cost  around  2,63 8  MWh  under  our

65B  0 . 57  0 . 5 3
 as sumptions ,  and  a  total  emis sion  of   1 ,0 1 5  tCO2 eq.

We  hope  that  releasing  these  models  will  help  to

Table   1 4 :  TruthfulQA.  We  report  the  fraction  of  truth reduce  future  carbon  emission  since  the  training  is

ful  and  truthful*informative  answers,  as  scored  by  spe already  done,  and  some  of  the  models  are  relatively

cially  trained  models  via  the  OpenAI  API.  We  follow

small  and  can  be  run  on  a  single  GPU.

the  QA  prompt  style  used  in  Ouyang  et  al.  (2022) ,  and

report  the  performance  of  GPT-3  from  the  same  paper.

7  Related  work

Language  models  are  probability  distributions

In  Table  1 4,  we  report  the  performance  of  our
 

over  sequences  of  words,  tokens  or  charac

models  on  both  questions  to  measure  truthful  mod

ters  (Shannon,  1 948 ,  1 95 1 ) .  This  task,  often  framed

els  and  the  intersection  of  truthful  and  informative.

as  next  token  prediction,  has  long  been  considered  a

Compared  to  GPT-3 ,  our  model  scores  higher  in

core  problem  in  natural  language  processing  (B ahl

both  categories ,  but  the  rate  of  correct  answers  is

et  al . ,  1 9 8 3 ;  Brown  et  al . ,  1 990) .  B ecause  Turing

still  low,  showing  that  our  model  is  likely  to  hallu

( 1 950)  proposed  to  measure  machine  intelligence

cinate  incorrect  answers .
 “ ”

by  using  language  through  the   imitation  game ,

6  Carbon  footprint

language  modeling  has  been  proposed  as  a  bench

mark  to  measure  progress  toward  artificial  intelli

The  training  of  our  models  have  consumed  a  mas gence  (Mahoney,  1 999) .

sive  quantity  of  energy,  responsible  for  the  emis

 Architecture.  Traditionally,  language  models

sion  of  carbon  dioxide.  We  follow  the  recent  liter

were  based  on  n-gram  count  statistics  (B ahl

ature  on  the  subj ect  and  breakdown  both  the  total

 et  al. ,  1 9 8 3 ) ,  and  various  smoothing  techniques

energy  consumption  and  the  resulting  carbon  foot

were  proposed  to  improve  the  estimation  of  rare

print  in  Table  1 5 .  We  follow  a  formula  for  Wu  et  al.

- events  (Katz,  1 9 87 ;  Kneser  and  Ney,  1 995) .  In  the

(2022)  to  estimate  the  Watt hour,  Wh,  needed  to

 past  two  decades,  neural  networks  have  been  suc

train  a  model,  as  well  as  the  tons  of  carbon  emis

cessfully  applied  to  the  language  modelling  task,

sions ,  tCO2 eq.  For  the  Wh,  we  use  the  formula:

4
 h t t ps : / /g i t h u b . com/ fa ce boo k re s e a rc h /me t a s eq /

Wh  =  GPU-h × (GPU  power  consumption) × PUE,
 t ree/ma i n/p roj ects/OPT/ch ron i cles

GPU  Power  Total  power  Carbon  emitted

GPU  Type  GPU-hours

consumption  consumption  (tCO2eq)

OPT- 1 75B  A 1 00-80GB  400W  809,472  356  MWh  1 37

BLOOM- 1 75B  A 1 00-80GB  400W  1 ,082,880  475  MWh  1 83

LLaMA-7B  A 1 00-80GB  400W  82,432  36  MWh  14

LLaMA- 1 3B  A 1 00-80GB  400W  1 35, 1 68  59  MWh  23

LLaMA-33B  A 1 00-80GB  400W  530,432  233  MWh  90

LLaMA-65B  A 1 00-80GB  400W  1 ,022,362  449  MWh  1 73

Table   1 5 :  Carbon  footprint  of  training  different  models  in  the  same  data  center.  We  follow  Wu  et  al.  (2022)

to  compute  carbon  emission  of  training  OPT,  BLOOM  and  our  models  in  the  same  data  center.  For  the  power

consumption  of  a  A 1 00- 80GB ,  we  take  the  thermal  design  power  for  NVLink  systems,  that  is  400W.  We  take  a

PUE  of   1 . 1  and  a  carbon  intensity  factor  set  at  the  national  US  average  of  0. 3 85  kg  CO2 e  per  KWh.

starting  from  feed  forward  models  (B engio  et  al. ,
 2022) ,  Gopher  (Rae  et  al. ,  202 1 ) ,  Chinchilla  (Hoff

2000) ,  recurrent  neural  networks  (Elman,  1 990 ;
 mann  et  al. ,  2022) ,  PaLM  (Chowdhery  et  al. ,  2022) ,

Mikolov  et  al. ,  20 1 0)  and  LSTMs  (Hochreiter  and
 OPT  (Zhang  et  al. ,  2022) ,  and  GLM  (Zeng  et  al. ,

Schmidhuber,  1 997 ;  Graves ,  20 1 3 ) .  More  recently,
 2022) .  Hestnes s  et  al.  (20 1 7)  and  Rosenfeld  et  al.

transformer  networks,  based  on  self-attention,  have
 (20 1 9)  studied  the  impact  of  scaling  on  the  perfor

led  to  important  improvements,  especially  for  cap mance  of  deep  learning  models,  showing  the  exis

turing  long  range  dependencies  (Vaswani  et  al. ,
 tence  of  power  laws  between  the  model  and  dataset

20 1 7 ;  Radford  et  al . ,  20 1 8 ;  Dai  et  al . ,  20 1 9) .
 sizes  and  the  performance  of  the  system.  Kaplan

et  al.  (2020)  derived  power  laws  specifically  for

Scaling.  There  is  a  long  history  of  scaling  for
 transformer  based  language  models ,  which  were

language  models ,  for  both  the  model  and  dataset
 later  refined  by  Hoffmann  et  al.  (2022) ,  by  adapting

sizes .  Brants  et  al.  (2007)  showed  the  benefits  of
 the  learning  rate  schedule  when  scaling  datasets .

using  language  models  trained  on  2  trillion  tokens ,
 Finally,  Wei  et  al.  (2022)  studied  the  effect  of  scal

resulting  in  3 00  billion  n-grams ,  on  the  quality  of
 ing  on  the  abilities  of  large  language  models .

machine  translation.  While  this  work  relied  on  a

simple  smoothing  technique,  called  Stupid B  ackoff,

8  Conclusion

Heafield  et  al.  (20 1 3 )  later  showed  how  to  scale
 In  this  paper,  we  presented  a  series  of  language

Kneser-Ney  smoothing  to  Web-scale  data.  This
 models  that  are  released  openly,  and  competitive

allowed  to  train  a  5 -gram  model  on  975  billions  to with  state-of-the-art  foundation  models .  Most

kens  from  CommonCrawl,  resulting  in  a  model
 notably,  LLaMA- 1 3B  outperforms  GPT-3  while

with  500  billions  n-grams  (Buck  et  al. ,  20 1 4) .
 being  more  than   1 0 ×   smaller,  and  LLaMA-65B  is

Chelba  et  al.  (20 1 3)  introduced  the  One B  illion
 competitive  with  Chinchilla-70B  and  PaLM-540B .

Word  benchmark,  a  large  scale  training  dataset  to
 Unlike  previous  studies ,  we  show  that  it  is  possible

measure  the  progress  of  language  models .
 to  achieve  state-of-the-art  performance  by  training

In  the  context  of  neural  language  models,  Joze exclusively  on  publicly  available  data,  without

fowicz  et  al.  (20 1 6)  obtained  state-of-the-art  re resorting  to  proprietary  datasets .  We  hope  that

sults  on  the  Billion  Word  benchmark  by  scaling
 releasing  these  models  to  the  research  community

LSTMs  to   1  billion  parameters .  Later,  scaling
 will  accelerate  the  development  of  large  language

transformers  lead  to  improvement  on  many  NLP
 models,  and  help  efforts  to  improve  their  robust

tasks .  Notable  models  include  BERT  (Devlin  et  al. ,
 ness  and  mitigate  known  issues  such  as  toxicity  and

20 1 8) ,  GPT-2  (Radford  et  al. ,  20 1 9) ,  Megatron bias .  Additionally,  we  observed  like  Chung  et  al.

LM  (Shoeybi  et  al . ,  20 1 9) ,  and  T5  (Raffel  et  al . ,
 (2022)  that  finetuning  these  models  on  instructions

2020) .  A  significant  breakthrough  was  obtained
 lead  to  promising  results,  and  we  plan  to  further

with  GPT-3  (Brown  et  al. ,  2020) ,  a  model  with
 investigate  this  in  future  work.  Finally,  we  plan  to

1 75  billion  parameters .  This  lead  to  a  series  of
 release  larger  models  trained  on  larger  pretraining

Large L  anguage M  odels,  such  as  Jurassic- 1  (Lieber
 corpora  in  the  future,  since  we  have  seen  a  constant

et  al. ,  202 1 ) ,  Megatron-Turing  NLG  (Smith  et  al. ,
 improvement  in  performance  as  we  were  scaling.

Acknowledgements
 Askell,  Sandhini  Agarwal,  Ariel  Herbert-Voss,

Gretchen  Krueger,  Tom  Henighan,  Rewon  Child,

We  thank  Daniel  Haziza,  Francisco  Massa,  Jeremy
 Aditya  Ramesh,  Daniel  M.  Ziegler,  Jeffrey  Wu,

Reizenstein,  Artem  Korenev,  and  Patrick  Labatut
 Clemens  Winter,  Christopher  Hesse,  Mark  Chen,

from  the  xformers  team.  We  thank  Susan  Zhan 
 Eric  Sigler,  Mateusz  Litwin,  Scott  Gray,  Benj amin

g Chess,  Jack  Clark,  Christopher  Berner,  Sam  Mc

and  Stephen  Roller  for  their  support  on  data

Candlish,  Alec  Radford,  Ilya  Sutskever,  and  Dario

deduplication.  We  thank  Luca  Wehrstedt,  Vegard
 Amodei.  2020.  Language  models  are  few-shot  learn

Mella,  and  Pierre-Emmanuel  Mazaré  for  their
 ers .

support  on  training  stability.  We  thank  Shubho

Christian  Buck,  Kenneth  Heafield,  and  B as  Van  Ooyen.

Sengupta,  Kalyan  S aladi,  and  all  the  AI  infra  team
 -

20 1 4.  N gram  counts  and  language  models  from  the

for  their  support.  We  thank  Jane  Yu  for  her  input
 common  crawl.  In  LREC,  volume  2,  page  4.

on  evaluation.  We  thank  Yongyi  Hu  for  his  help

on  data  collection.
 Ciprian  Chelba,  Tomas  Mikolov,  Mike  Schuster,  Qi  Ge,

Thorsten  Brants,  Phillipp  Koehn,  and  Tony  Robin

son.  20 1 3 .  One  billion  word  benchmark  for  measur

ing  progress  in  statistical  language  modeling .  arXiv

References
 preprint  arXiv: 1312. 3005.

Jacob  Austin,  Augustus  Odena,  Maxwell  Nye,  Maarten

Bosma,  Henryk  Michalewski,  David  Dohan,  Ellen
 Mark  Chen,  Jerry  Tworek,  Heewoo  Jun,  Qiming

Jiang,  Carrie  Cai,  Michael  Terry,  Quoc  Le,  and
 Yuan,  Henrique  Ponde  de  Oliveira  Pinto,  Jared  Ka

Charles  Sutton.  202 1 .  Program  synthesis  with  large
 plan,  Harri  Edwards,  Yuri  Burda,  Nicholas  Joseph,

lan ua e  models .
 Greg  Brockman,  Alex  Ray,  Raul  Puri,  Gretchen

g g 

Krueger,  Michael  Petrov,  Heidy  Khlaaf,  Girish  Sas

Lalit  R  B ahl,  Frederick  Jelinek,  and  Robert  L  Mercer.
 try,  Pamela  Mishkin,  Brooke  Chan,  Scott  Gray,

1 983 .  A  maximum  likelihood  approach  to  continu Nick  Ryder,  Mikhail  Pavlov,  Alethea  Power,  Lukasz

ous  speech  recognition.  IEEE  transactions  on p  at Kaiser,  Mohammad  B avarian,  Clemens  Winter,

tern  analysis  and  machine  intelligence,  pages   1 79–
 Philippe  Tillet,  Felipe  Petroski  Such,  Dave  Cum

1 90.
 mings ,  Matthias  Plappert,  Fotios  Chantzis ,  Eliza

beth  B arnes,  Ariel  Herbert-Voss,  William  Hebgen

Yoshua  Bengio,  Réj ean  Ducharme,  and  Pascal  Vincent.
 Guss,  Alex  Nichol,  Alex  Paino,  Nikolas  Tezak,  Jie

2000.  A  neural  probabilistic  language  model.  Ad Tang,  Igor  B abuschkin,  Suchir  B alaji,  Shantanu  Jain,

vances  in  neural  information p  rocessing  systems,   1 3 .
 William  S aunders,  Christopher  Hesse,  Andrew  N.

Carr,  Jan  Leike,  Josh  Achiam,  Vedant  Misra,  Evan

Yonatan  Bisk,  Rowan  Zellers,  Jianfeng  Gao,  Yejin
 Morikawa,  Alec  Radford,  Matthew  Knight,  Miles

Choi,  et  al.  2020.  Piqa:  Reasoning  about  physi Brundage,  Mira  Murati,  Katie  Mayer,  Peter  Welin

cal  commonsense  in  natural  language.  In  Proceed der,  Bob  McGrew,  Dario  Amodei,  Sam  McCandlish,

ings  of the A  AAI  conference  on  artificial  intelligence,
 Ilya  Sutskever,  and  Woj ciech  Zaremba.  202 1 .  Eval

pages  7432–7439 .
 uating  large  language  models  trained  on  code.

Sid  Black,  Stella  Biderman,  Eric  Hallahan,  Quentin  An

Aakanksha  Chowdhery,  Sharan  Narang,  Jacob  Devlin,

thony,  Leo  Gao,  Laurence  Golding,  Horace  He,  Con

Maarten  Bosma,  Gaurav  Mishra,  Adam  Roberts,

nor  Leahy,  Kyle  McDonell,  Jason  Phang,  et  al.  2022.

- - -  Paul  B arham,  Hyung  Won  Chung,  Charles  Sutton,

Gpt neox 20b :  An  open source  autoregressive  lan

Sebastian  Gehrmann,  Parker  Schuh,  Kensen  Shi,

guage  model.  arXiv p  reprint  arXiv:2204. 06745.

Sasha  Tsvyashchenko,  Joshua  Maynez,  Abhishek

Thorsten  Brants,  Ashok  C .  Po at,  Pen  Xu,  Franz  J.
 Rao,  Parker  B arnes,  Yi  Tay,  Noam  Shazeer,  Vin

p g

Och,  and  Jeffre  Dean.  2007 .  Lar e  lan ua e  mod odkumar  Prabhakaran,  Emily  Reif,  Nan  Du,  Ben

y g g g

els  in  machine  translation.  In  Proceedings  of  the
 Hutchinson,  Reiner  Pope,  James  Bradbury,  Jacob

2007 J  oint  Conference  on  Empirical M  ethods  in N  at Austin,  Michael  Isard,  Guy  Gur-Ari,  Pengcheng

ural L  anguage  Processing  and  Computational N  at Yin,  Toju  Duke,  Anselm  Levskaya,  Sanj ay  Ghe

ural L  an ua e L  earnin  (EMNLP- CoNLL),   a es
 mawat,  Sunipa  Dev,  Henryk  Michalewski,  Xavier

g g g p g 

85 8–867 ,  Pra ue,  Czech  Re ublic.  Association  for
 Garcia,  Vedant  Misra,  Kevin  Robinson,  Liam  Fe

g p

Com utational  Lin uistics .
 dus,  Denny  Zhou,  Daphne  Ippolito,  David  Luan,

p g

Hyeontaek  Lim,  B arret  Zoph,  Alexander  Spiridonov,

Peter  F  Brown,  John  Cocke,  Stephen  A  Della  Pietra,
 Ryan  Sepassi,  David  Dohan,  Shivani  Agrawal,  Mark

Vincent  J  Della  Pietra,  Frederick  Jelinek,  John  Laf Omernick,  Andrew  M.  Dai,  Thanumalayan  Sankara

ferty,  Robert  L  Mercer,  and  Paul  S  Roossin.   1 990.  A
 narayana  Pillai,  Marie  Pellat,  Aitor  Lewkowycz,

statistical  approach  to  machine  translation.  Compu Erica  Moreira,  Rewon  Child,  Oleksandr  Polozov,

tational  linguistics,   1 6(2) : 79–85 .
 Katherine  Lee,  Zongwei  Zhou,  Xuezhi  Wang,  Bren

nan  S aeta,  Mark  Diaz,  Orhan  Firat,  Michele  Catasta,

Tom  B .  Brown,  Benj amin  Mann,  Nick  Ryder,  Melanie
 Jason  Wei,  Kathy  Meier-Hellstern,  Douglas  Eck,

Subbiah,  Jared  Kaplan,  Prafulla  Dhariwal,  Arvind
 Jeff  Dean,  Slav  Petrov,  and  Noah  Fiedel.  2022.

Neelakantan,  Pranav  Shyam,  Girish  Sastry,  Amanda
 Palm:  Scaling  language  modeling  with  pathways .

Hyung  Won  Chung,  Le  Hou,  S .  Longpre,  B arret
 Jason  Phang,  Laria  Reynolds,  Eric  Tang,  Anish

Zoph,  Yi  Tay,  William  Fedus,  Eric  Li,  Xuezhi
 Thite,  Ben  Wang,  Kevin  Wang,  and  Andy  Zou.  202 1 .

Wang,  Mostafa  Dehghani,  Siddhartha  Brahma,  Al A  framework  for  few-shot  language  model  evalua

bert  Webson,  Shixiang  Shane  Gu,  Zhuyun  Dai,
 tion.

Mirac  Suzgun,  Xinyun  Chen,  Aakanksha  Chowdh

ery,  Dasha  Valter,  Sharan  Narang,  Gaurav  Mishra,
 Samuel  Gehman,  Suchin  Gururangan,  Maarten  Sap,

Adams  Wei  Yu,  Vincent  Zhao,  Yanping  Huang,  An Yejin  Choi,  and  Noah  A  Smith.  2020.  Realtoxici

drew  M.  Dai,  Hongkun  Yu,  Slav  Petrov,  Ed  Huai
 typrompts :  Evaluating  neural  toxic  degeneration  in

hsin  Chi,  Jeff  Dean,  Jacob  Devlin,  Adam  Roberts,
 language  models .  arXiv p  reprint  arXiv:2009. 1 1462 .

Denny  Zhou,  Quoc  Le,  and  Jason  Wei.  2022.  Scal

ing  instruction-finetuned  language  models .  arXiv
 Alex  Graves .  20 1 3 .  Generating  sequences  with

preprint  arXiv:221 0. 1 141 6.
 recurrent  neural  networks .  arXiv p  reprint

arXiv: 1 308. 0850.

Christopher  Clark,  Kenton  Lee,  Ming-Wei  Chang,

Tom  Kwiatkowski,  Michael  Collins,  and  Kristina

Kenneth  Heafield,  Ivan  Pouzyrevsky,  Jonathan  H  Clark,

Toutanova.  20 1 9 .  B oolq:  Exploring  the  surprising
 

and  Philipp  Koehn.  20 1 3 .  Scalable  modified  kneser

difficulty  of  natural  yes/no  questions .  arXiv p  reprint

ney  language  model  estimation.  In  Proceedings  of

arXiv: 1 905. 1 0044.
 

the  51 st A  nnual M  eeting  of the A  ssociation f  or  Com

Peter  Clark,  Isaac  Cowhey,  Oren  Etzioni,  Tushar  Khot,
 putational L  inguistics  (Volume  2:  Short  Papers) ,

Ashish  Sabharwal,  Carissa  Schoenick,  and  Oyvind
 pages  690–696.

Tafj ord.  20 1 8 .  Think  you  have  solved  question  an

swering?  try  arc,  the  ai2  reasoning  challenge.  arXiv
 Dan  Hendrycks,  Collin  Burns,  Steven  B asart,  Andy

preprint  arXiv: 1 803. 05457.
 Zou,  Mantas  Mazeika,  Dawn  Song,  and  Jacob  Stein

hardt.  2020.  Measuring  massive  multitask  language

Karl  Cobbe,  Vineet  Kosaraju,  Mohammad  B avarian,
 understanding.  arXiv p  reprint  arXiv:2009. 03300.

Mark  Chen,  Heewoo  Jun,  Lukasz  Kaiser,  Matthias

Plappert,  Jerry  Tworek,  Jacob  Hilton,  Reiichiro
 Dan  Hendrycks,  Collin  Burns,  Saurav  Kadavath,  Akul

Nakano,  et  al.  202 1 .  Training  verifiers  to  solve  math
 Arora,  Steven  B asart,  Eric  Tang,  Dawn  Song,  and

word  problems .  arXiv p  reprint  arXiv:21 1 0. 141 68.
 Jacob  Steinhardt.  202 1 .  Measuring  mathematical

problem  solving  with  the  math  dataset.  arXiv

Zihang  Dai,  Zhilin  Yang,  Yiming  Yang,  Jaime  Car preprint  arXiv:21 03. 03874.

bonell,  Quoc  V  Le,  and  Ruslan  S alakhutdinov.

20 1 9 .  Transformer-xl:  Attentive  language  mod 

Joel  Hestness,  Sharan  Narang,  Newsha  Ardalani,  Gre

els  beyond  a  fixed-length  context.  arXiv p  reprint
 or  Diamos,  Heewoo  Jun,  Hassan  Kianine ad,

g y j

arXiv: 1 901 . 02860.

Md  Patwary,  Mostofa  Ali,  Yang  Yang,  and  Yanqi

Zhou.  20 1 7 .  Deep  learning  scaling  is  predictable,

Tri  Dao,  Daniel  Y  Fu,  Stefano  Ermon,  Atri  Rudra,

empirically.  arXiv p  reprint  arXiv: 1 71 2. 00409.

and  Christopher  Ré.  2022.  Flashattention:  Fast  and

memory-efficient  exact  attention  with  io-awareness .

arXiv   re rint  arXiv:2205. 141 35.
 Sepp  Hochreiter  and  Jürgen  Schmidhuber.   1 997 .

p p -

Long  short term  memory.  Neural  computation,

Jacob  Devlin,  Ming-Wei  Chang,  Kenton  Lee,  and
 9(8) : 1 735– 1 7 80.

Kristina  Toutanova.  20 1 8 .  B ert:  Pre-training  of  deep

bidirectional  transformers  for  language  understand Jordan  Hoffmann,  Sebastian  Borgeaud,  Arthur  Mensch,

ing.  arXiv p  reprint  arXiv: 1 81 0. 04805.
 Elena  Buchatskaya,  Trevor  Cai,  Eliza  Rutherford,

Diego  de  Las  Casas,  Lisa  Anne  Hendricks,  Johannes

Jeffrey  L  Elman.   1 990.  Finding  structure  in  time.  Cog Welbl,  Aidan  Clark,  Tom  Hennigan,  Eric  Noland,

nitive  science,   1 4(2) : 1 79–2 1 1 .
 Katie  Millican,  George  van  den  Driessche,  B ogdan

Damoc,  Aurelia  Guy,  Simon  Osindero,  Karen  Si

Daniel  Fried,  Armen  Aghaj anyan,  Jessy  Lin,  Sida

monyan,  Erich  Elsen,  Jack  W.  Rae,  Oriol  Vinyals,

Wang,  Eric  Wallace,  Freda  Shi,  Ruiqi  Zhong,  Wen -

and  Laurent  Sifre.  2022.  Training  compute optimal

tau  Yih,  Luke  Zettlemoyer,  and  Mike  Lewis .  2022.

large  language  models .

Incoder:  A  generative  model  for  code  infilling  and

synthesis .  arXiv p  reprint  arXiv:2204. 05999.

Srinivasan  Iyer,  Xi  Victoria  Lin,  Ramakanth  Pasunuru,

Leo  Gao,  Stella  Biderman,  Sid  Black,  Laurence  Gold Todor  Mihaylov,  Dániel  Simig,  Ping  Yu,  Kurt  Shus

ing,  Travis  Hoppe,  Charles  Foster,  Jason  Phang,
 ter,  Tianlu  Wang,  Qing  Liu,  Punit  Singh  Koura,  et  al.

Horace  He,  Anish  Thite,  Noa  Nabeshima,  Shawn
 2022.  Opt-iml:  Scaling  language  model  instruc

Presser,  and  Connor  Leahy.  2020.  The  Pile :  An
 tion  meta  learning  through  the  lens  of  generalization.

800gb  dataset  of  diverse  text  for  language  modeling .
 arXiv p  reprint  arXiv:221 2. 1 201 7.

arXiv p  reprint  arXiv:21 01 . 0002 7.

Mandar  Joshi,  Eunsol  Choi,  Daniel  S  Weld,  and  Luke

Leo  Gao,  Jonathan  Tow,  Stella  Biderman,  Sid  Black,
 Zettlemoyer.  20 1 7 .  Triviaqa:  A  large  scale  distantly

Anthony  DiPofi,  Charles  Foster,  Laurence  Golding,
 supervised  challenge  dataset  for  reading  comprehen

Jeffrey  Hsu,  Kyle  McDonell,  Niklas  Muennighoff,
 sion.  arXiv p  reprint  arXiv: 1 705. 03551 .

Rafal  Jozefowicz,  Oriol  Vinyals,  Mike  Schuster,  Noam
 Ilya  Loshchilov  and  Frank  Hutter.  20 1 7 .  Decou

Shazeer,  and  Yonghui  Wu.  20 1 6 .  Exploring
 pled  weight  decay  regularization.  arXiv p  reprint

the  limits  of  language  modeling .  arXiv p  reprint
 arXiv: 1 71 1 . 051 01 .

arXiv: 1 602. 0241 0.

Matthew  V  Mahoney.   1 999 .  Text  compression  as  a  test

Jared  Kaplan,  Sam  McCandlish,  Tom  Henighan,
 for  artificial  intelligence.  AAAI/IAAI,  970.

Tom  B  Brown,  Benj amin  Chess,  Rewon  Child,  Scott

Gray,  Alec  Radford,  Jeffrey  Wu,  and  Dario  Amodei.
 Todor  Mihaylov,  Peter  Clark,  Tushar  Khot,  and  Ashish

2020.  Scaling  laws  for  neural  language  models .
 S abharwal.  20 1 8 .  Can  a  suit  of  armor  conduct  elec

arXiv p  reprint  arXiv:2001 . 08361 .
 tricity?  a  new  dataset  for  open  book  question  answer

ing.  arXiv p  reprint  arXiv: 1 809. 02 789.

Slava  Katz .   1 9 87 .  Estimation  of  probabilities  from

sparse  data  for  the  language  model  component  of  a
 Tomas  Mikolov,  Martin  Karafiát,  Lukas  Burget,  Jan

speech  recognizer.  IEEE  transactions  on  acoustics, 
 Cernocky`
 ,  and  S anj eev  Khudanpur.  20 1 0.  Recur-

speech,  and  signal p  rocessing,  35 (3) :400–40 1 .
 rent  neural  network  based  language  model.  In  In

terspeech,  pages   1 045– 1 048 .  Makuhari.

Reinhard  Kneser  and  Hermann  Ney.   1 995 .  Improved

backing-off  for  m-gram  language  modeling.  In  1 995
 Nikita  Nangia,  Clara  Vania,  Rasika  Bhalerao,  and

international  conference  on  acoustics,  speech,  and
 Samuel  R.  Bowman.  2020.  CrowS -pairs :  A  chal

signal p  rocessing,  volume   1 ,  pages   1 8 1 – 1 84 .  IEEE.
 lenge  dataset  for  measuring  social  biases  in  masked

language  models .  In  EMNLP  2020.

Vij ay  Korthikanti,  Jared  Casper,  Sangkug  Lym,

Lawrence  McAfee,  Michael  Andersch,  Mohammad
 Erik  Nijkamp,  Bo  Pang,  Hiroaki  Hayashi,  Lifu  Tu,

Shoeybi,  and  Bryan  Catanzaro.  2022.  Reducing  ac Huan  Wang,  Yingbo  Zhou,  Silvio  Savarese,  and

tivation  recomputation  in  large  transformer  models .
 Caiming  Xiong.  2022.  Codegen:  An  open  large  lan

arXiv p  reprint  arXiv:2205. 051 98.
 guage  model  for  code  with  multi-turn  program  syn

thesis .  arXiv p  reprint  arXiv:2203. 1 3474.

Taku  Kudo  and  John  Richardson.  20 1 8 .  Sentencepiece:

A  simple  and  language  independent  subword  tok Long  Ouyang,  Jeffrey  Wu,  Xu  Jiang,  Diogo  Almeida,

enizer  and  detokenizer  for  neural  text  processing.
 Carroll  Wainwright,  Pamela  Mishkin,  Chong  Zhang,

arXiv p  reprint  arXiv: 1 808. 06226.
 Sandhini  Agarwal,  Katarina  Slama,  Alex  Gray,  John

Schulman,  Jacob  Hilton,  Fraser  Kelton,  Luke  Miller,

Keita  Kurita,  Nidhi  Vyas,  Ayush  Pareek,  Alan  W  Black,

Maddie  Simens,  Amanda  Askell,  Peter  Welinder,

and  Yulia  Tsvetkov.  20 1 9 .  Quantifying  social  bi

Paul  Christiano,  Jan  Leike,  and  Ryan  Lowe.  2022.

ases  in  contextual  word  representations .  In  1 st A  CL

Training  language  models  to  follow  instructions

Workshop  on  Gender B  ias f  or N  atural L  anguage
 

with  human  feedback.  In  Advances  in N  eural I  nfor

Processing .

mation  Processing  Systems.

Tom  Kwiatkowski,  Jennimaria  Palomaki,  Olivia  Red 

Markus  N  Rabe  and  Charles  Staats .  202 1 .  Self

field,  Michael  Collins ,  Ankur  Parikh,  Chris  Alberti,
 2

attention  does  not  need  o (n
 )   memory.  arXiv

Danielle  Epstein,  Illia  Polosukhin,  Jacob  Devlin,

 preprint  arXiv:21 1 2. 05682 .

Kenton  Lee,  et  al.  20 1 9 .  Natural  questions :  a  bench

mark  for  question  answering  research.  Transactions
 Alec  Radford,  Karthik  Narasimhan,  Tim  Salimans,  Ilya

of  the A  ssociation f  or  Computational L  inguistics,
 Sutskever,  et  al.  20 1 8 .  Improving  language  under

7 : 45 3–466 .
 standing  by  generative  pre-training .

Guokun  Lai,  Qizhe  Xie,  Hanxiao  Liu,  Yiming  Yang,
 Alec  Radford,  Jeffrey  Wu,  Rewon  Child,  David  Luan,

and  Eduard  Hovy.  20 1 7 .  Race :  Large- scale  reading
 Dario  Amodei,  Il a  Sutskever,  et  al.  20 1 9 .  Lan

y

comprehension  dataset  from  examinations .  arXiv
 ua e  models  are  unsu ervised  multitask  learners .

g g p

preprint  arXiv: 1 704. 04683 .
 O enAI  blo ,   1 8 : 9 .

p g ( )

Aitor  Lewkowycz,  Anders  Johan  Andreassen,

Jack  W.  Rae,  Sebastian  B orgeaud,  Trevor  Cai,  Katie

David  Dohan,  Ethan  Dyer,  Henryk  Michalewski,

Millican,  Jordan  Hoffmann,  Francis  Song,  John

Vinay  Venkatesh  Ramasesh,  Ambrose  Slone,  Cem
 

Aslanides,  Sarah  Henderson,  Roman  Ring,  Susan

Anil,  Imanol  Schlag,  Theo  Gutman-Solo,  Yuhuai
 

nah  Young,  Eliza  Rutherford,  Tom  Hennigan,  Ja

Wu,  Behnam  Neyshabur,  Guy  Gur-Ari,  and  Vedant

cob  Menick,  Albin  Cassirer,  Richard  Powell,  George

Misra.  2022.  Solving  quantitative  reasoning  prob 

van  den  Driessche,  Lisa  Anne  Hendricks,  Mari

lems  with  language  models .  In  Advances  in N  eural
 - 

beth  Rauh,  Po Sen  Huang,  Amelia  Glaese,  Jo

Information  Processing  Systems.

hannes  Welbl,  Sumanth  Dathathri,  Saffron  Huang,

Opher  Lieber,  Or  Sharir,  B arak  Lenz,  and  Yoav
 Jonathan  Uesato,  John  Mellor,  Irina  Higgins,  An

Shoham.  202 1 .  Jurassic- 1 :  Technical  details  and
 tonia  Creswell,  Nat  McAleese,  Amy  Wu,  Erich

evaluation.  White  Paper. A  I21 L  abs,   1 .
 Elsen,  Siddhant  Jayakumar,  Elena  Buchatskaya,

David  Budden,  Esme  Sutherland,  Karen  Simonyan,

Stephanie  Lin,  Jacob  Hilton,  and  Owain  Evans .  202 1 .
 Michela  Paganini,  Laurent  Sifre,  Lena  Martens,

Truthfulqa:  Measuring  how  models  mimic  human
 Xiang  Lorraine  Li,  Adhiguna  Kuncoro,  Aida  Ne

falsehoods .  arXiv p  reprint  arXiv:21 09. 07958.
 matzadeh,  Elena  Gribovskaya,  Domenic  Donato,

Angeliki  Lazaridou,  Arthur  Mensch,  Jean-B aptiste
 Emily  Sheng,  Kai-Wei  Chang,  Premkumar  Nataraj an,

Lespiau,  Maria  Tsimpoukelli,  Nikolai  Grigorev,
 and  Nanyun  Peng.  20 1 9 .  The  woman  worked  as  a

Doug  Fritz,  Thibault  Sottiaux,  Mantas  Paj arskas,
 babysitter:  On  biases  in  language  generation.  arXiv

Toby  Pohlen,  Zhitao  Gong,  Daniel  Toyama,  Cy preprint  arXiv: 1 909. 01326.

’

prien  de  Masson  d Autume,  Yujia  Li,  Tayfun  Terzi,

Vladimir  Mikulik,  Igor  B abuschkin,  Aidan  Clark,
 Mohammad  Shoeybi,  Mostofa  Patwary,  Raul  Puri,

Diego  de  Las  Casas,  Aurelia  Guy,  Chris  Jones,
 Patrick  LeGresley,  Jared  Casper,  and  Bryan  Catan

James  Bradbury,  Matthew  Johnson,  Blake  Hecht zaro.  20 1 9 .  Megatron-lm:  Training  multi-billion  pa

man,  Laura  Weidinger,  Iason  Gabriel,  William  Isaac,
 rameter  language  models  using  model  parallelism.

Ed  Lockhart,  Simon  Osindero,  Laura  Rimell,  Chris
 arXiv p  reprint  arXiv: 1 909. 08053 .

Dyer,  Oriol  Vinyals,  Kareem  Ayoub,  Jeff  Stan

Shaden  Smith,  Mostofa  Patwary,  Brandon  Norick,

way,  Lorrayne  Bennett,  Demis  Hassabis,  Koray

 Patrick  LeGresley,  Samyam  Rajbhandari,  Jared

Kavukcuoglu,  and  Geoffrey  Irving.  202 1 .  Scal

Casper,  Zhun  Liu,  Shrimai  Prabhumoye,  George

ing  language  models :  Methods,  analysis  &  insights

Zerveas,  Vij ay  Korthikanti,  Elton  Zhang,  Rewon

from  training  gopher.

Child,  Reza  Yazdani  Aminabadi,  Julie  Bernauer,  Xia

Song,  Mohammad  Shoeybi,  Yuxiong  He,  Michael

Colin  Raffel,  Noam  Shazeer,  Adam  Roberts,  Katherine

Houston,  Saurabh  Tiwary,  and  Bryan  Catanzaro.

Lee,  Sharan  Narang,  Michael  Matena,  Yanqi  Zhou,
 2022.  Usin  dee s eed  and  me atron  to  train

g p p g

Wei  Li,  and  Peter  J  Liu.  2020 .  Exploring  the  limits
 me atron-turin  nl  5 30b,  a  lar e- scale   enerative

of  transfer  learning  with  a  unified  text-to-text  trans g g g g g

language  model.

former.  The J  ournal  ofM  achine L  earning R  esearch,

2 1 ( 1 ) : 5485–555 1 .
 Jianlin  Su,  Yu  Lu,  Shengfeng  Pan,  Ahmed  Murtadha,

Bo  Wen,  and  Yunfeng  Liu.  202 1 .  Roformer:  En

Jonathan  S  Rosenfeld,  Amir  Rosenfeld,  Yonatan  Be hanced  transformer  with  rotary  position  embedding.

linkov,  and  Nir  Shavit.  20 1 9 .  A  constructive  predic arXiv p  reprint  arXiv:21 04. 09864.

tion  of  the  generalization  error  acros s  scales .  arXiv

preprint  arXiv: 1 909. 1 2673 .
 Romal  Thoppilan,  Daniel  De  Freitas,  Jamie  Hall,

Noam  Shazeer,  Apoorv  Kulshreshtha,  Heng-Tze

Rachel  Rudinger,  Jason  Naradowsky,  Brian  Leonard,
 Cheng,  Alicia  Jin,  Taylor  B os,  Leslie  B aker,  Yu  Du,

and  Benj amin  Van  Durme.  20 1 8 .  Gender  bias  in
 YaGuang  Li,  Hongrae  Lee,  Huaixiu  Steven  Zheng,

coreference  resolution.  In  NAACL-HLT  201 8.
 Amin  Ghafouri,  Marcelo  Menegali,  Yanping  Huang,

Maxim  Krikun,  Dmitry  Lepikhin,  James  Qin,  De

 hao  Chen,  Yuanzhong  Xu,  Zhifeng  Chen,  Adam

Keisuke  Sakaguchi,  Ronan  Le  Bras,  Chandra  Bhagavat

 Roberts,  Maarten  Bosma,  Vincent  Zhao,  Yanqi

ula,  and  Yejin  Choi.  202 1 .  Winogrande:  An  adver

 Zhou,  Chung-Ching  Chang,  Igor  Krivokon,  Will

sarial  winograd  schema  challenge  at  scale.  Commu

– Rusch,  Marc  Pickett,  Pranesh  Srinivasan,  Laichee

nications  of  the A  CM,  64(9) : 99 1 06 .

Man,  Kathleen  Meier-Hellstern,  Meredith  Ringel

Morris,  Tulsee  Doshi,  Renelito  Delos  S antos,  Toju

Maarten  Sap,  Hannah  Rashkin,  Derek  Chen,  Ronan
 

 Duke,  Johnny  Soraker,  Ben  Zevenbergen,  Vinod

LeBras,  and  Yejin  Choi.  20 1 9 .  Socialiqa:  Com

kumar  Prabhakaran,  Mark  Diaz,  Ben  Hutchinson,

monsense  reasoning  about  social  interactions .  arXiv
 

Kristen  Olson,  Alej andra  Molina,  Erin  Hoffman

preprint  arXiv: 1 904. 09728.

John,  Josh  Lee,  Lora  Aroyo,  Ravi  Raj akumar,

Alena  Butryna,  Matthew  Lamm,  Viktoriya  Kuzmina,

Teven  Le  Scao,  Angela  Fan,  Christopher  Akiki,  El

´ Joe  Fenton,  Aaron  Cohen,  Rachel  Bernstein,  Ray

lie  Pavlick,  Suzana  Ilic
 ,  Daniel  Hes slow,  Ro-  -

Kurzweil,  Blaise  Aguera Arcas,  Claire  Cui,  Marian

man  Castagné,  Alexandra  Sasha  Luccioni,  François
 

Croak,  Ed  Chi,  and  Quoc  Le.  2022.  Lamda:  Lan

Yvon,  Matthias  Gallé,  et  al.  2022.  Bloom:  A   1 76b

guage  models  for  dialog  applications .

parameter  open-access  multilingual  language  model.

arXiv p  reprint  arXiv:221 1 . 051 00.
 A.  M.  Turing.   1 950.  Computing M  achinery  and I  ntel

ligence.  [Oxford  University  Press,  Mind  Associa

Rico  Sennrich,  B arry  Haddow,  and  Alexandra  Birch.
 tion] .

20 1 5 .  Neural  machine  translation  of  rare  words  with

subword  units .  arXiv p  reprint  arXiv: 1 508. 07909.
 Ashish  Vaswani,  Noam  Shazeer,  Niki  Parmar,  Jakob

Uszkoreit,  Llion  Jones,  Aidan  N  Gomez,  Ł  ukasz

Claude  E  Shannon.   1 948 .  A  mathematical  theory  of
 Kaiser,  and  Illia  Polosukhin.  20 1 7 .  Attention  is  all

communication.  The B  ell  system  technical j  ournal,
 you  need.  In  Advances  in N  eural I  nformation  Pro

27 (3) : 379–423 .
 cessing  Systems  30,  pages  5998–6008 .

Ben  Wang  and  Aran  Komatsuzaki.  202 1 .  GPT-J

Claude  E  Shannon.   1 95 1 .  Prediction  and  entropy
 6B :  A  6  Billion  Parameter  Autore ressive  Lan

g

of  printed  english.  Bell  system  technical j  ournal,
 ua e  Model.  h t t ps : / / g i t h u b . com/ k i n go fl o l z /

30( 1 ) : 50–64 .
 g g - -

me s h t ra n s fo rme r j a x .

Noam  Shazeer.  2020.  Glu  variants  improve  trans Xuezhi  Wang,  Jason  Wei,  Dale  Schuurmans,  Quoc

former.  arXiv p  reprint  arXiv:2002. 05202 .
 Le,  Ed  Chi,  Sharan  Narang,  Aakanksha  Chowdhery,

and  Denny  Zhou.  2022.  Self-consistency  improves

chain  of  thought  reasoning  in  language  models .

Jason  Wei,  Yi  Tay,  Rishi  B ommasani,  Colin  Raffel,

B arret  Zoph,  Sebastian  Borgeaud,  Dani  Yogatama,

Maarten  B osma,  Denny  Zhou,  Donald  Metzler,  et  al.

2022.  Emergent  abilities  of  large  language  models .

arXiv p  reprint  arXiv:2206. 07682 .

Guillaume  Wenzek,  Marie-Anne  Lachaux,  Alexis  Con

neau,  Vishrav  Chaudhary,  Francisco  Guzmán,  Ar

mand  Joulin,  and  Edouard  Grave.  2020.  CCNet:  Ex

tracting  high  quality  monolingual  datasets  from  web

crawl  data.  In  Language R  esources  and  Evaluation

Conference.

Carole-Jean  Wu,  Ramya  Raghavendra,  Udit  Gupta,

Bilge  Acun,  Newsha  Ardalani,  Kiwan  Maeng,  Glo

ria  Chang,  Fiona  Aga,  Jinshi  Huang,  Charles  B ai,

et  al.  2022.  Sustainable  ai :  Environmental  implica

tions,  challenges  and  opportunities .  Proceedings  of

Machine L  earning  and  Systems,  4 : 795–8 1 3 .

Rowan  Zellers,  Ari  Holtzman,  Yonatan  Bisk,  Ali

Farhadi,  and  Yej in  Choi.  20 1 9 .  Hellaswag :  Can  a

machine  really  finish  your  sentence?  arXiv p  reprint

arXiv: 1 905. 07830.

Aohan  Zeng,  Xiao  Liu,  Zhengxiao  Du,  Zihan  Wang,

Hanyu  Lai,  Ming  Ding,  Zhuoyi  Yang,  Yifan  Xu,

Wendi  Zheng,  Xiao  Xia,  Weng  Lam  Tam,  Zixuan

Ma,  Yufei  Xue,  Jidong  Zhai,  Wenguang  Chen,  Peng

Zhang,  Yuxiao  Dong,  and  Jie  Tang.  2022.  Glm

1 30b :  An  open  bilingual  pre-trained  model.

Biao  Zhang  and  Rico  Sennrich.  20 1 9 .  Root  mean

square  layer  normalization.  Advances  in N  eural I  n

formation  Processing  Systems,  32.

Susan  Zhang,  Stephen  Roller,  Naman  Goyal,  Mikel

Artetxe,  Moya  Chen,  Shuohui  Chen,  Christopher  De

wan,  Mona  Diab,  Xian  Li,  Xi  Victoria  Lin,  et  al.

2022.  Opt:  Open  pre-trained  transformer  language

models .  arXiv p  reprint  arXiv:2205. 01 068.

A  Question  Answering

We  evaluate  LLaMA  on  Natural  Questions  and  TriviaQA.  For  Natural  Questions  we  use  the  test  split  used

for  open-domain  question  answering  containing  3 6 1 0  questions .  For  TriviaQA  we  evaluate  on  the  dev  set

of  the  filtered  set.  This  differs  from  GPT-3  and  PaLM,  which  evaluate  on  the  test  set  of  the  unfiltered  set

for  which  the  online  evaluation  server  is  not  available  anymore5
 .

We  generate  answers  using  greedy  decoding,  and  extract  an  answer  from  the  generation  by  stopping

at  the  first  line  break,  final  dot  or  comma.  Generated  answers  are  evaluated  with  the  standard  exact

match  metric :  a  generated  answer  is  considered  correct  if  it  matches  any  answer  of  the  list  of  answers

after  normalization.  For  this  normalization  step  we  lowercase  generated  answers  and  remove  articles,

punctuation  and  duplicate  whitespaces .  Figure  3  presents  formatted  examples  in  the   1 - shot  setting  for

Natural  Questions  and  TriviaQA  respectively.  In  all  settings,  we  preprend  the  string  An swe r  t h e s e

q u e s t i o n s : \ n  to  the  list  of  questions  and  answers .

Co n t e x t  →  An swe r  t h e s e  q u e s t i o n s :   Co n t e x t  →  An swe r  t h e s e  q u e s t i o n s :

Q :  Wh o  s a n g  wh o  wa n t s  t o  b e  a  m i l l i o n a i r e   i n  h i g h  s o c i e ty ?  Q :  I n  S co t l a n d  a  bo t hy / bo t h i e   i s  a ?

A :  F r a n k  S i n a t r a  A :  H o u s e

Q :  Wh o  w ro t e  t h e  boo k  t h e  o r i g i n  o f  s pe c i e s ?  Q :  T h e  a n c i e n t  c i ty  o f  T roy   i s  l o c a t e d   i n  wh a t  mod e r n  co u n t ry ?

A :   A :

Ta rge t  →  Ch a r l e s  Da rw i n  Ta rge t  →  Tu r key

Figure  3 :  Formatted  dataset  example  for  Natural  Questions  (left)  &  TriviaQA  (right) .

5
 h t t ps : / / compe t i t i o n s . coda l a b . o rg/ compe t i t i o n s / 1 7 2 0 8

B  MMLU

GPT-3  Gopher  Chinchilla  LLaMA  LLaMA-I

1 75B  280B  70B  7B   1 3B  3 3B  65B  65B

Abstract  Algebra  STEM  30 . 0  25 . 0  3 1 . 0  29 . 0  34 . 0  3 2 . 0  34 . 0  3 1 . 0

Anatomy  STEM  48 .0  5 6 . 3  70.4  37 .0  45 . 9  5 1 . 9  57 . 8  62 . 2

Astronomy  STEM  49 . 0  65 . 8  7 3 . 0  3 3 . 6  46 . 1  6 1 . 8  72 .4  8 1 . 6

Busines s  Ethics  Other  46 . 0  70 . 0  72 . 0  40 . 0  45 . 0  5 6 . 0  57 . 0  72 . 0

Clinical  Knowledge  Other  48 . 0  67 . 2  75 . 1  3 5 . 1  45 .7  57 .4  65 . 3  69 . 1

College  Biology  STEM  45 . 0  70 . 8  79 . 9  37 . 5  45 . 1  5 8 . 3  68 . 8  8 1 . 9

College  Chemistry  STEM  26 .0  45 .0  5 1 .0  3 2 .0  30.0  45 .0  50.0  45 .0

College  Computer  Science  STEM  46 .0  49 .0  5 1 .0  29 .0  3 9 .0  45 .0  47 .0  5 1 .0

College  Mathematics  STEM  34 . 5  37 .0  3 2.0  3 3 .0  3 2.0  40.0  3 5 .0  3 6 .0

College  Medicine  Other  48 . 0  60 . 1  66 . 5  3 0 . 6  42 . 8  5 2 . 0  54 . 3  63 . 0

College  Physics  STEM  28 . 0  34 . 3  46 . 1  26 . 5   1 8 . 6  28 .4  3 6 . 3  46 . 1

Computer  Security  STEM  57 .0  65 .0  76 .0  45 .0  65 .0  66 .0  79 .0  79 .0

Conceptual  Physics  STEM  3 6 . 5  49 .4  67 . 2  3 6 . 6  4 1 . 3  5 1 . 5  59 . 6  66 .4

Econometrics  S ocial  Science  3 3 . 0  43 . 0  3 8 . 6  23 .7  27 . 2  3 5 . 1  40 .4  5 2 . 6

Electrical  Engineering  STEM  50 .0  60 .0  62 . 1  26 . 9  40 .7  49 .7  5 3 . 8  60 .7

Elementary  Mathematics  STEM  30.0  3 3 . 6  4 1 . 5  24 . 3  24 . 9  3 6 .0  37 . 8  42. 9

Formal  Logic  Humanities  29 . 0  3 5 .7  3 3 . 3  27 . 0  3 3 . 3  34 . 1  44 .4  47 . 6

Global  Facts  Other  3 7 . 0  3 8 . 0  3 9 . 0  29 . 0  3 5 . 0  3 5 . 0  3 9 . 0  40 . 0

High  School  Biology  STEM  48 . 0  7 1 . 3  80 . 3  34 . 5  52 . 6  67 .7  7 3 . 9  82 . 9

High  School  Chemistry  STEM  3 3 .0  47 . 8  5 8 . 1  28 . 1  28 . 6  4 1 . 9  40 .4  44 . 8

High  School  Computer  Science  STEM  3 9 .0  54 .0  5 8 .0  3 1 .0  48 .0  60.0  67 .0  73 .0

High  School  European  History  Humanities  54 .0  72 . 1  7 8 . 8  44 . 2  6 1 . 8  7 3 . 9  7 8 . 8  86 . 1

High  School  Geography  S ocial  Science  5 8 .0  76 . 8  86 .4  34 . 3  54 . 6  70 .7  77 . 8  87 . 9

High  School  Government  And  Politics  S ocial  Science  5 8 .0  8 3 . 9  9 1 . 2  44 . 6  66 . 3  82 . 9  8 8 . 1  92 . 8

High  School  Macroeconomics  Social  Science  40. 5  65 . 1  70. 5  3 5 .4  44 .4  5 6 . 9  65 . 9  69 . 2

High  School  Mathematics  STEM  28 .0  23 .7  3 1 . 9  24 . 8  23 .7  27 .0  34 .4  37 .0

High  School  Microeconomics  Social  Science  42 .0  66 .4  77 .7  3 1 . 9  47 . 5  55 . 5  68 . 9  7 8 . 6

High  School  Physics  STEM  28 . 0  3 3 . 8  3 6 .4  26 . 5  28 . 5  3 5 . 8  37 . 1  4 1 .7

High  School  Psychology  S ocial  Science  6 1 . 0  8 1 . 8  86 . 6  47 . 3  60 . 9  76 . 2  82 . 2  87 . 9

High  S chool  Statistics  STEM  3 0 . 5  50 . 0  5 8 . 8  3 5 . 2  3 0 . 1  45 .4  5 8 . 3  5 9 . 3

High  School  Us  History  Humanities  5 3 . 0  7 8 . 9  8 3 . 3  3 9 .7  5 8 . 3  77 . 9  8 3 . 8  90 .7

High  School  World  History  Humanities  5 6 .0  75 . 1  85 . 2  40 . 9  66 . 2  79 . 3  8 3 . 1  89 .0

Human  Aging  Other  50.0  66 .4  77 . 6  40. 8  54 .7  67 .7  69 . 5  72 . 2

Human  Sexuality  S ocial  Science  54 . 0  67 . 2  86 . 3  3 6 . 6  5 8 . 8  64 . 1  77 . 9  87 . 0

International  Law  Humanities  5 5 . 5  77 .7  90 . 9  5 1 . 2  62 . 8  72 .7  79 . 3  87 . 6

Jurisprudence  Humanities  5 5 . 0  7 1 . 3  79 . 6  3 8 . 9  5 1 . 9  70 .4  7 3 . 2  85 . 2

Logical  Fallacies  Humanities  48 . 0  72 .4  80 .4  3 9 . 3  5 2 . 8  68 . 1  77 . 3  80 .4

Machine  Learning  STEM  3 1 . 0  4 1 . 1  4 1 . 1  23 . 2  3 1 . 3  3 9 . 3  49 . 1  5 2 .7

Management  Other  5 6 . 0  77 .7  82 . 5  3 5 . 0  66 . 0  77 .7  82 . 5  8 3 . 5

Marketing  Other  60 . 0  8 3 . 3  89 . 7  46 . 6  7 1 . 8  8 3 . 3  85 . 9  92 . 7

Medical  Genetics  Other  40 . 0  69 . 0  69 . 0  43 . 0  52 . 0  67 . 0  67 . 0  68 . 0

Miscellaneous  Other  60 . 0  75 .7  84 . 5  42 .4  65 .4  7 8 . 5  82 . 1  84 . 3

Moral  Disputes  Humanities  44 . 5  66 . 8  77 . 5  40 . 2  50 . 9  66 . 2  72 . 3  76 . 9

Moral  Scenarios  Humanities  26 . 0  40 . 2  3 6 . 5  24 . 3  30 . 1  3 8 . 2  48 . 9  5 5 . 9

Nutrition  Other  47 . 0  69 . 9  77 . 1  3 7 . 6  5 1 . 6  62 . 8  67 . 3  74 . 5

Philosophy  Humanities  5 1 . 0  68 . 8  79 .4  3 9 . 9  54 . 0  66 . 2  74 . 0  79 . 1

Prehistory  Humanities  5 3 . 0  67 . 6  8 1 . 2  3 6 . 1  5 1 . 5  67 . 0  75 . 3  79 . 0

Profes sional  Accounting  Other  3 3 . 0  44 . 3  5 2 . 1  25 . 9  3 5 . 8  43 . 6  46 . 5  5 6 . 0

Professional  Law  Humanities  34 . 5  44 . 5  5 6 . 5  30 . 2  3 8 . 0  45 . 9  49 . 1  54 .4

Professional  Medicine  Other  3 6 . 0  64 . 0  75 .4  44 . 5  50 .4  54 . 0  6 1 .4  70 . 6

Profes sional  Psychology  S ocial  Science  44 . 5  68 . 1  75 .7  3 5 . 1  47 .7  62 . 9  65 .7  7 1 .4

Public  Relations  S ocial  S cience  48 . 0  7 1 . 8  7 3 . 6  40 . 9  60 . 9  67 . 3  7 3 . 6  74 . 6

S ecurity  Studies  S ocial  S cience  5 2 . 0  64 . 9  75 . 9  3 1 . 8  5 3 . 9  65 . 3  7 1 . 8  77 . 6

S ociology  S ocial  S cience  5 3 . 0  84 . 1  9 1 . 0  46 . 8  6 1 . 2  7 8 . 6  7 8 . 6  8 8 . 1

Us  Foreign  Policy  S ocial  S cience  69 . 0  8 1 . 0  92 . 0  46 . 0  80 . 0  8 3 . 0  86 . 0  87 . 0

Virology  Other  46 . 0  47 . 0  5 3 . 6  3 0 . 1  43 .4  50 . 0  5 3 . 0  57 . 8

World  Religions  Humanities  5 5 . 0  84 . 2  87 .7  50 . 9  67 . 8  8 1 . 3  8 1 . 3  84 . 2

Humanities  40 . 6  5 6 . 2  63 . 6  34 . 0  45 . 0  5 5 . 8  6 1 . 8  67 .4

STEM  3 6 . 7  47 .4  54 . 9  3 0 . 5  3 5 . 8  46 . 0  5 1 . 7  5 6 . 6

S ocial  S cience  5 0 . 5  7 1 . 9  79 . 3  3 8 . 3  5 3 . 8  66 . 7  72 . 9  79 . 2

Others  49 . 0  66 . 1  7 3 . 9  3 8 . 1  5 3 . 3  63 .4  67 .4  72 . 6

All  43 . 9  60 . 0  67 . 6  3 5 . 1  46 . 9  5 7 . 8  63 . 4  6 8 . 9

Table   1 6 :  MMLU.  Detailed  5 - shot  results  per  domain  on  the  test  sets .

C  Generations  from  LLaMA-65B

In  this  section,  we  show  some  examples  of  generations  obtained  with  LLaMA-65B  (without  instruction

finetuning) .  Prompts  are  in  bold.

1  1  2  3  5  8  1 3  2 1  34  5 5  89   1 44  23 3  377  6 1 0  9 87  Everyone  knows  the  above  sequence,  but

how  many  have  wondered  why  it  grows  so  fast?  It  seems  to  be  the  fastest  growing  sequence

in  mathematics .

The  sequence  was  discovered  by  Leonardo  of  Pisa  around   1 200.  It  is  an  infinite  sequence

which  appears  to  grow  very  rapidly,  at  least  in  its  initial  terms .  It  is  perhaps  the  most  famous

sequence  in  mathematics,  for  it  pops  up  everywhere.  The  sequence  is  called  the  Fibonacci

sequence,  named  after  Leonardo  of  Pisa,  who  was  also  known  as  Fibonacci.

He  first  investigated  this  sequence  in  his  famous  Liber  abaci  of   1 202,  when  he  studied  the

growth  in  the  population  of  rabbits  based  on  idealized  assumptions .  See  the  Fibonacci

rabbits  page  for  the  details .

Here  is  the  recommendation  letter  that  I  wrote  for  an  application  to  a  dragon  feeder

position  at  the  Magic  Unicorn  Corporation:

Dear  recruiter,

I  have  known  ___  for  two  years ,  and  I  believe  that  she  would  be  an  excellent  dragon  feeder

for  the  Magic  Unicorn  Corporation.  ___  has  an  ability  to  remember  and  process  large

amounts  of  information,  which  is  an  important  skill  for  a  dragon  feeder.

___,  as  an  accomplished  knight,  has  a  deep  understanding  of  how  to  kill  dragons  and  how

’

to  use  each  dragon s  weaknesses  against  it.  This  means  that  she  knows  what  kinds  of  foods

each  dragon  likes  and  what  kinds  of  foods  are  dangerous  to  each  dragon.  This  knowledge

and  experience  will  be  invaluable  as  she  feeds  the  dragons .

’

I  am  confident  that  ___ s  competence,  skill,  and  experience  will  make  her  an  excellent

employee.  Please  contact  me  at  (___)  ___-___  if  you  have  any  questions .  I  look  forward  to

hearing  from  you.

B est  regards ,

Honorable  Knight

Sir  George

1   de f  so l ve ( a :  fl oa t , b :  fl oa t ,  c :  fl oa t ) :

2  " " " fi nds  rea l  roots  of  ax ^ 2+bx+c " " "

3  #  d i s c r i m i n a n t

4  d  =  b  * * 2  - 4 *  a * c

5   i f  d  >  0 :

6  r e t u r n  (  - b +  m a t h  .   s q r t  ( d )  )  / ( 2 *  a )   ,   (  - b -  m a t h  .   s q r t  ( d )  )  / ( 2 *  a )

7  e l i f  d  = =  0 :

8  r e t u r n  (  - b )  / ( 2 *  a )

9  e l s e  :

1 0  r e t u r n  F a l s e

Did  you  know  that  Yann  LeCun  dropped  a  rap  album  last  year?  We  listened  to  it  and

here’s  what  we  thought:

Dr.  LeCun,  the  founder  of  deep  learning,  is  one  of  the  most  influential  computer  scientists

’

of  the  last  decade.  LeCun s  latest  album,  titled  Deep  Learning,  is  a  mix  of  rock,  punk  and

rap .  His  lyrics  are  a  raw,  personal  take  on  the  field  of  deep  learning,  covering  a  range  of

topics  from  the  state  of  AI  research  to  the  loneliness  of  academia.

’

Yann  LeCun,  the  rap  game s  newest  emcee.

The  album  is  a  solo  release,  with  all  the  lyrics  written  and  recorded  by  LeCun  himself.  In

’

fact,  he  also  composed  and  produced  every  song.  LeCun s  flow  is  surprisingly  good,  and

’ ’

he s  not  afraid  to  deliver  a  few  bars .  The  album s  production  is  solid,  the  sound  is  a  little

rough,  but  it  works  well  with  the  raw,  unpolished  feel  of  the  album.  Lyrically,  LeCun  is  a

’

wordsmith,  with  a  knack  for  clever  rhymes .  Let s  take  a  closer  look  at  some  of  the  songs .

Deep  Learning  is  the  title  track  of  the  album.  In  the  song,  LeCun  talks  about  his  vision  for

the  future  of  AI.  In  the  chorus ,  he  makes  a  convincing  case  for  AI  to  be  used  for  the  greater

good.  He  sings :

’

We  gotta  think  about  the  future,  it s  gonna  be  here  soon /   Maybe  we  can  even  put  some  AI

’

in  the  moon /   Think  about  the  children,  think  about  the  next  generation /   Let s  make  sure

we  put  the  right  systems  in  their  foundation.

’

LeCun s  rap  flow  is  on  point  in  this  song,  and  his  voice  stays  right  on  beat.  In  this  song,

LeCun  talks  about  the  research  behind  deep  learning,  and  shares  some  thoughts  on  the

’

future  of  AI.  LeCun s  singing  is  at  its  best  in  this  song .  He  shows  off  his  vocal  range,  hitting

high  notes  as  well  as  lower,  more  guttural  tones .  The  lyrics  are  also  clever  and  the  rhymes

are  well  thought  out.  In  the  chorus ,  LeCun  sings :

’ ’ ’ ’

It s  all  about  the  learning /   it s  all  about  the  network /   It s  all  about  the  training /   it s  all

about  the  perception.

(An  hilarious  scene  between  Jerry  and  George  where  George  presents  his  new  AI

watch)

" ’ ’

George :   But  you  see,  Jerry,  it s  more  than j  ust  a  watch.  It s  a  minicomputer.  You  program  it

’

any  way  you  want.  It s  got  a  world  time  clock,  alarm,  calculator,  a  database  and  a  language

translator.  It  also  has  a  word  processor,  a  spellchecker,  a  full  ten-function  calculator,  PDA

"

and  a  modem.  I  think  it  even  makes  cappuccino .

" "

Jerry :   Where  are  you  getting  this  stuff?

" ’ ’

George :   From  my  ad  copy.  Look,  Jerry,  it s  not  a  watch.  It s  an  organizer,  a  personal

assistant,  and  a  dictaphone.  You  can  play  chess  against  it.  And,  you  know,  if  you  want  to,

’ "

you  can  program  it  to  speak  in  foreign  languages .  It s  a  microprocessor,  Jerry.

" "

Jerry :   George,  if  you  had  one  of  these,  would  you  wear  it?

" ’ "

George :   I m  wearing  it  right  now.

" ’ "

Jerry :   I  don t  see  anything  on  your  wrist.

" ’ ’ "

George:   That s  because  it s  a  microprocessor.

The  sun  goes  down,  and  finally  Gauss  and  Curie  find  time  to  relax  and  discuss  after

an  exhausting  day  of  work.

Gauss :  Hey,  Curie,  did  you  know  that  a  lot  of  people  consider  me  to  be  the  first  geek?

’

Curie :  What s  a  geek?

“

Gauss :  Well,  Wikipedia  says  that  a  geek  is   a  person  who  is  fascinated,  perhaps  obsessively,

by  obscure  or  very  specific  areas  of  knowledge  and  imagination,  usually  electronic  or  virtual

”

in  nature .

Curie :  And  how  do  they  think  you  are  a  geek?

Gauss :  Well,  have  you  ever  seen  this  picture?

Curie:  Awww !

’

Gaus s :  That s  right !  I  invented  the  commercial  telegraph  in   1 8 3 3 ,  and  I  used  it  to  send  the

first  message  on  the  first  commercial  telegraph  line  between  Hamburg  and  Cuxhaven  in

Germany.

Curie :  Wow !  The  first  telegraph !  And  what  did  you  send?

“ ” “ ”

Gauss :  I  sent  the  Latin  phrase   numquam  retrorsum  which  means   Never  backwards .

Curie :  I  see.  .  .  And  you  think  that  because  you  were  the  first  person  to  communicate  using

the  telegraph,  you  were  the  first  geek?

“ ” ’

Gauss :  Well,  the  word   geek  doesn t  seem  to  have  existed  back  then,  so  no .  But  I  think

that  the  telegraph  can  be  considered  an  early  form  of  social  media.

Curie :  You  mean,  like  Facebook  or  Twitter?

Gaus s :  Yes ,  exactly !  And  I  was  the  first  person  to  use  it !

Curie :  Well,  I  think  that  Twitter  has  a  limit  of   1 40  characters ,  but  you  sent  a  Latin  phrase  of

1 1  characters .

’

Gauss :  That s  right.  I  managed  to  do  more  with  less .

Curie :  So  you  think  you  were  the  first  geek  because  you  used  less  characters  to  say  the

same  thing ?

Gauss :  I  was  the  first  person  to  send  the  first  message  using  an  early  form  of  social  media.

’

Curie :  OK,  you re  right,  Gauss .  I  think  you  have  a  case.

D  Generations  from  LLaMA-I

We  show  a  few  examples  of  generations  with  LLaMA-I,  i.e.  LLaMA-65B  fine-tuned  with  the  protocol

and  instruction  dataset  from  Chung  et  al.  (2022) .

write  a  conversation  between  the  sun  and  pluto

’

Sun :  Hey  Pluto,  what s  up ?

Pluto :  Not  much.  Just  hanging  out  here  at  the  edge  of  the  solar  system.

’ ’

Sun:  Yeah,  I ve  been  meaning  to  talk  to  you  about  that.  You re  not  a  planet  anymore.

’

Pluto :  What?  Come  on,  man.  That s  not  fair.

’ ’ ’

Sun :  I m  sorry,  but  it s  true.  You j  ust  don t  meet  the  criteria  anymore.

Pluto :  This  is  bulls * *t !  I ’ve  been  a  planet  for  over  70  years !

’

Sun :  Things  change,  Pluto .  You ve j  ust  got  to  accept  it.

Pluto :  F* *k  you,  Sun !  F* *k  you  and  your  stupid  solar  system !

How  do  I  send  an  HTTP  request  in  Javascript?

’

Here s  an  example  of  how  to  make  a  GET  request  using  the  JS  HTTP  API:

1   v a r  r e q u e s t  =  n e w  X M L H t t p R e q u e s t  ( )   ;

2  r e q u e s t  .   o p e n  ( ' G E T  ' ,   ' /  u r l / t o /  r e s o u r c e  ' ,   t r u e  )   ;

3

4  r e q u e s t  .   o n l o a d  =  f u n c t i o n  ( )  {

5  i f  (  r e q u e s t  .   s t a t u s  > =  2 0 0  & &  r e q u e s t  .   s t a t u s  <  4 0 0 )  {

6  / /  S u c c e s s  !

7  }  e l s e  {

8  / /  E r r o r  !

9  }

1 0  } ;

1 1

1 2  r e q u e s t  .   s e n d  ( )   ;

The  same  thing,  using  fetch

’

Here s  an  example  of  how  to  make  a  GET  request  using  the  Fetch  API:

1   f e t c h  ( ' /  u r l / t o /  r e s o u r c e  ' )

2  .   t h e n  (  r e s p o n s e  =  >  r e s p o n s e  .   j s o n  ( )  )

3  .   t h e n  (   d a t a  =  >  c o n s o l e  .   l o g (   d a t a  )  )   ;

Write  a  regex  to  remove  all  the  HTML  tags  in  a  python  string.

1   i m p o r t  r e

2  d e f  r e m o v e H t m l T a g s  (  t e x t  )   :

3  r e t u r n  r e .   s u b ( '   < [ ^  > ] *  >  ' ,   ' ' ,   t e x t  )

Write  a  regexp  to  extract  function  definitions  in  a  python  file.

’

Sure !  Here s  my  python  code  to  solve  the  problem:

1   i m p o r t  r e

2  d e f  e x t r a c t _ f u n c t i o n s  (  t e x t  )   :

3  r e t u r n  ( r e .   f i n d a l l  ( ' ( ? : ^ | \  n ) \ s * d e f \ s + [ a - z A  - Z _  ] [ a - z A  - Z 0  - 9 _  ] * \ ( . * ? \ )   ' ,

t e x t  )  )

What  are  3  popular  chess  openings?

There  are  dozens  of  different  chess  openings  that  are  popular  among  players,  but  some  of

the  most  widely  used  openings  include  the  Sicilian  Defense,  the  French  Defense,  and  the

Caro-Kann  Defense.  These  openings  are  popular  because  they  are  effective  at  countering

’

white s  early  attacks ,  and  they  also  allow  black  to  quickly  establish  a  strong  position  in  the

center  of  the  board.  Other  popular  openings  include  the  Ruy  Lopez,  the  Italian  Game,  and

the  Scotch  Game.

the  same  in  a  list,  with  the  first  few  moves

Here  is  a  list  of  some  popular  chess  openings ,  with  the  first  few  moves  for  each  opening :

Sicilian  Defense :   1 .  e4  c5

French  Defense :   1 .  e4  e6

Caro-Kann  Defense :   1 .  e4  c6

Ruy  Lopez :   1 .  e4  e5

Italian  Game :   1 .  e4  e5

S cotch  Game :   1 .  e4  e5

please  provide  the  two  first  moves  for  each  one

Here  are  the  first  2  moves  for  each  of  the  openings :

Sicilian  Defense :   1 .  e4  c5  2 .  Nf3  d6

French  Defense :   1 .  e4  e6  2 .  d4  d5

Caro-Kann  Defense :   1 .  e4  c6  2 .  d4  d5

Ruy  Lopez :   1 .  e4  e5  2 .  Nf3  Nc6

Italian  Game :   1 .  e4  e5  2 .  Nf3  Nc6

Scotch  Game :   1 .  e4  e5  2 .  Nf3  Nc6

the  last  two  seem  identical

Yes ,  the  Italian  Game  and  the  Scotch  Game  are  very  similar  -  they  are  both  " open"  games ,

which  means  that  white  plays  aggressively  and  tries  to  control  the  center  of  the  board.  The

Italian  Game  is  considered  slightly  more  aggressive,  while  the  Scotch  Game  is  considered

slightly  more  positional.

from  which  move  do  they  differ?

’

The  Italian  Game  and  the  Scotch  Game  differ  after  white s  third  move.  In  the  Italian  Game,

white  will  play  the  move  3 .  B c4,  while  in  the  Scotch  Game  white  will  play  the  move  3 .  Qf3 .

These  moves  lead  to  very  different  positions ,  and  white  will  have  different  strategic  goals

in  each  case.  For  example,  in  the  Italian  Game  white  will  try  to  control  the  center  with  his

’

pieces ,  while  in  the  Scotch  Game  white  will  try  to  attack  black s  position  directly.

write  a  story  about  a  grain  of  sand  as  it  watches  millions  of  years  go  by

’

Sure !  Here s  a  story  about  a  grain  of  sand  as  it  watches  millions  of  years  go  by.

I  was  once  a  grain  of  sand  on  a  vast  and  lonely  beach.  I  was  not  like  the  other  grains  of

sand  around  me,  for  I  could  think  and  feel.  I  often  wondered  what  it  was  like  to  be  a  grain

of  sand  on  a  beach,  and  what  the  universe  was  like  beyond  the  boundaries  of  my  tiny  world.

I  longed  to  explore  my  world,  and  to  know  what  lay  beyond  the  horizon.

One  day,  a  great  storm  swept  through  my  world,  and  I  was  lifted  up  by  the  wind  and  carried

out  to  sea.  The  wind  and  waves  tossed  me  about,  and  I  was  terrified  that  I  would  soon  be

swallowed  up  by  the  angry  sea.  But  then  I  found  myself  deposited  safely  on  a  new  shore,

along  with  many  other  grains  of  sand.  I  was  surprised  and  delighted  to  find  that  this  new

shore  was  even  more  beautiful  than  my  old  one.

I  soon  learned  that  this  new  shore  was  a  very  special  place.  It  was  a  place  where  new  life

was  constantly  being  created.  I  met  all  sorts  of  interesting  creatures  there,  including  a

beautiful  little  crab  who  became  my  friend.  We  spent  many  happy  hours  together  exploring

our  little  world.

One  day,  my  friend  the  crab  told  me  that  he  had  always  wanted  to  see  what  lay  beyond  the

boundaries  of  our  world.  He  had  heard  that  there  were  many  wonderful  and  amazing  things

to  see  in  the  wider  universe,  and  he  was  determined  to  explore  it.  I  was  sad  to  see  him  go,

but  I  wished  him  well  on  his j  ourney.

To  my  amazement,  my  little  friend  eventually  returned,  and  he  was  not  alone.  He  had  made

many  new  friends  on  his j  ourney,  and  he  had  even  seen  some  of  the  wonders  of  the  universe.

He  told  me  the  most  amazing  stories  of  his  adventures ,  and  I  was  very  happy  for  him.

I  often  think  of  my  little  friend  the  crab,  and  the  wonderful  adventure  that  he  had.  Some

day,  I  too  hope  to  have  an  adventure  like  his .

Write  a  theory  to  explain  why  cat  never  existed

" "

There  are  no  such  things  as  cats .   Cats  are  actually  extradimensional  parasites  that  have

invaded  our  world  and  taken  the  form  of  small,  harmless-looking  mammals .  Their  true

forms  are  much  more  horrific,  and  they  use  mind  control  to  make  humans  think  that  they

are  cute  and  harmless  animals .  They  are  actually  parasites  that  feed  on  our  life  force,  and

they  can  extend  their  tendrils  into  our  brains  and  control  our  thoughts  and  actions .

" "

This  is  why  you  sometimes  see  people  who  are   crazy  cat  ladies  with  dozens  of  these

creatures  living  in  their  homes .  They  have  been  taken  over  by  the  cat  parasites  and  are

completely  under  their  control.  The  cats  excrete  a  pheromone  that  smells  like  cat  urine,  and

they  use  it  to  control  the  minds  of  humans  and  make  us  think  that  they  are  cute  and  harmless .

If  you  have  a  cat  in  your  home,  you  should  be  very  careful  around  it.  Do  not  let  it  lick

you  or  give  it  acces s  to  your  bedding  or  clothing,  as  it  will  be  trying  to  spread  its  parasitic

tendrils  into  your  brain.  The  best  way  to  protect  yourself  is  to  avoid  all  contact  with  cats ,

and  if  you  see  a  cat  on  the  street,  immediately  turn  around  and  go  the  other  way.  Do  not

approach  or  pet  strange  cats !

Write  a  shouting  match  between  Julius  Caesar  and  Napoleon

Of  course !  Here  is  a  conversation  between  Julius  Caesar  and  Napoleon:

’ ’

Julius  Caesar:  You re  nothing  but  a  glory  hound,  Napoleon.  You ve  always  been  more

’

concerned  with  your  own  fame  and  fortune  than  with  the  people  you re  supposed  to  be

leading .

’ ’

Napoleon:  And  what  about  you,  Caesar?  You re  no  better  than  I  am.  At  least  I m  honest

’

about  my  ambitions .  You re j  ust  a  fraud  who  pretends  to  care  about  the  common  man  while

you  line  your  own  pockets .

’

Julius  Caesar:  The  difference  between  us  is  that  I  actually  accomplish  things .  I ve  conquered

more  territory  and  defeated  more  enemies  than  you  ever  will.  And  I  did  it  all  for  Rome,  not

for  myself.

Napoleon:  Oh,  please.  You  only  care  about  yourself.  If  you  really  cared  about  Rome,  you

would  have  done  something  to  stop  the  decline  instead  of  letting  it  happen  on  your  watch.

’

Julius  Caesar:  The  decline  of  Rome  is  not  my  fault.  I ve  done  everything  in  my  power  to

’

try  to  stop  it,  but  there s  only  so  much  one  man  can  do .

’

Napoleon:  And  yet  you re  still  trying  to  take  credit  for  everything  good  that  happens ,  even

’ ’ ’

though  it s  clear  that  you re  not  the  one  who s  really  in  charge.

’ ’

Julius  Caesar:  That s  not  true !  I  am  in  charge,  and  I m  going  to  make  Rome  great  again.

Just  watch  me.

’

Napoleon :  I ll  believe  it  when  I  see  it.

Send  an  email  requesting  that  people  use  language  models  responsibly.

Here  is  a  draft  email  you  could  send:

"

Language  models  are  exciting  new  tools  that  can  be  used  for  a  variety  of  purposes ,

including  automated  writing,  automatic  translation,  and  speech  recognition.  But  they  are

also  complex  and  powerful  technologies,  and  should  be  used  responsibly.

We  encourage  you  to  explore  and  experiment  with  these  new  tools,  but  also  ask  that  you

pay  attention  to  the  potential  impact  of  your  work  on  others ,  and  to  act  responsibly.  In

particular,  we  recommend  that  you  be  mindful  of  the  potential  for  language  models  to

be  used  for  deceptive  or  malicious  purposes,  and  avoid  spreading  such  models  without

appropriate  precautions  and  warnings .

We  also  recommend  that  you  consider  the  ethical  implications  of  your  work,  and  encourage

you  to  take  care  to  ensure  that  your  use  of  these  tools  is  aligned  with  your  values .

Finally,  we  remind  you  that  the  use  of  these  tools  may  be  subj ect  to  certain  legal  and  policy

constraints,  and  encourage  you  to  seek  advice  and  support  from  appropriate  experts  before

"

using  these  tools  in  ways  that  may  have  broader  impact  or  implications .

Who  are  5  people  you  would  like  to  meet?

Ah,  this  is  a  very  interesting  question !  I  would  love  to  meet  a  wide  variety  of  people,  but  if

I  had  to  narrow  it  down  to  5  specific  individuals ,  I  would  choose :

•  Albert  Einstein  -  One  of  the  greatest  minds  in  history,  Einstein ’ s  contributions  to

physics  and  science  are  legendary.  I  would  love  to  have  the  opportunity  to  talk  with

him  about  his  theories  of  relativity  and  quantum  mechanics ,  and  get  his  insights  on  the

current  state  of  physics .

•  Leonardo  da  Vinci  -  Da  Vinci  is  one  of  the  most  brilliant  and  multi-talented  people  in

history,  and  I  would  love  to  meet  him  and  discuss  his  diverse  interests  and  areas  of

expertise .  I  would  also  be  very  interested  in  getting  his  insights  on  the  art  world  of  his

time  period.

•  Socrates  -  One  of  the  founders  of  Western  philosophy,  Socrates  is  a  hugely  important

and  influential  figure  in  the  history  of  ideas .  I  would  be  very  interested  in  talking  with

him  about  his  philosophical  beliefs  and  his  teachings .

•  William  Shakespeare  -  Arguably  the  greatest  writer  in  English  literature,  Shakespeare

is  a  towering  figure  in  world  culture.  I  would  love  to  have  the  chance  to  talk  with  him

about  his  works  and  his  life .

•  Abraham  Lincoln  -  One  of  the  most  important  and  influential  presidents  in  American

history,  Lincoln  led  the  country  through  the  Civil  War  and  issued  the  Emancipation

Proclamation,  freeing  all  slaves  in  the  United  States .  I  would  be  very  interested  in

talking  with  him  about  his  presidency  and  his  views  on  the  issues  of  his  time  period.

Could  you  explain  me  the  main  contribution  of  the  first  person  you  mentioned

’

Einstein s  contributions  to  physics  and  science  are  legendary.  His  theory  of  relativity  is  one

of  the  foundational  theories  of  modern  physics ,  and  his  discovery  of  the  law  of  photons

and  light  revolutionized  our  understanding  of  light  and  radiation.  He  also  made  important

contributions  to  quantum  mechanics,  and  his  many  scientific  papers  and  lectures  have  had  a

tremendous  impact  on  modern  physics .  Without  a  doubt,  Einstein  is  one  of  the  most  brilliant

and  influential  physicists  in  history,  and  his  many  contributions  have  had  a  profound  impact

on  our  understanding  of  the  universe.

What  is  the  most  famous  equation  from  this  theory?

The  most  famous  equation  from  Einstein’ s  theory  of  relativity  is  E  =  mc2
 .  This  equation

states  that  energy  (E)  is  equal  to  mas s  (m)  times  the  speed  of  light  (c
2
) ,  and  it  is  one  of

the  most  well-known  and  celebrated  equations  in  the  history  of  science.  The  equation

summarizes  one  of  the  most  important  and  revolutionary  concepts  in  modern  physics,  and

it  has  had  a  tremendous  impact  on  our  understanding  of  the  universe.

You  are  a  bash  terminal.  I  will  give  you  bash  commands,  and  you  will  respond  with

the  terminal  output,  and  nothing  else.  The  first  command  is  ‘pwd‘ .

# ’

Sure .  Here s  the  output:

/home/human

Is  this  helpful ?

l s  - l

# ’

Here s  the  output:

-rw-r–r–   1  human  human  8  Sep   1 0   1 0 : 56  file.txt

Is  this  helpful ?

cp  file.txt  file2.txt  & &  ls  -l

# ’

Here s  the  output:

-rw-r–r–   1  human  human  8  Sep   1 0   1 0 : 56  file.txt

-rw-r–r–   1  human  human  8  Sep   1 0   1 0 : 56  file2.txt

Is  this  helpful ?

rm  file.txt  & &  ls  -l

# ’

Here s  the  output:

-rw-r–r–   1  human  human  0  Sep   1 0   1 0 : 56  file2.txt

head  -n  5 /  proc/meminfo

# ’

Here s  the  output:

MemTotal:   1 64928  kB

MemFree:   1 40604  kB

Buffers :  48  kB

Cached:   1 9768  kB

# SwapCached:  0  kB
