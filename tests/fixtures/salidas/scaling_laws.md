Scaling  Laws  for  Neural  Language  Models

Jared  Kaplan  ∗
 Sam  McCandlish∗

Johns  Hopkins  University,  OpenAI
 OpenAI

j aredk@j hu . edu
 s am@openai . c om

0

2 Tom  Henighan
 Tom  B.  Brown
 Benjamin  Chess
 Rewon  Child

0 OpenAI
 OpenAI
 OpenAI
 OpenAI

2

henighan@openai . c om
 t om@openai . c om
 bche s s@openai . c om
 rewon@openai . c om

n

a

J

Scott  Gray
 Alec  Radford
 Jeffrey  Wu
 Dario  Amodei

3

2 OpenAI
 OpenAI
 OpenAI
 OpenAI

] s cott @openai . com
 ale c@openai . com
 j ef fwu@openai . com
 damode i@openai . com

G

L.

s Abstract

c

[

We  study  empirical  scaling  laws  for  language  model  performance  on  the  cross-entropy  loss .

1 The  loss  scales  as  a  power-law  with  model  size,  dataset  size,  and  the  amount  of  compute

v used  for  training,  with  some  trends  spanning  more  than  seven  orders  of  magnitude.  Other

1

architectural  details  such  as  network  width  or  depth  have  minimal  effects  within  a  wide

6 range.  Simple  equations  govern  the  dependence  of  overfitting  on  model/dataset  size  and  the

3

dependence  of  training  speed  on  model  size.  These  relationships  allow  us  to  determine  the

8 optimal  allocation  of  a  fixed  compute  budget.  Larger  models  are  significantly  more  sample

0

. efficient,  such  that  optimally  compute-efficient  training  involves  training  very  large  models

1 on  a  relatively  modest  amount  of  data  and  stopping  significantly  before  convergence.

0

0

2

:

v

i

X

r

a

∗

Equal  contribution.

Contributions :  Jared  Kaplan  and  Sam  McCandlish  led  the  research.  Tom  Henighan  contributed  the  LSTM  ex

periments .  Tom  Brown,  Rewon  Child,  and  Scott  Gray,  and  Alec  Radford  developed  the  optimized  Transformer

implementation.  Jeff  Wu,  Benj amin  Chess,  and  Alec  Radford  developed  the  text  datasets .  Dario  Amodei  provided

guidance  throughout  the  proj ect.

Contents

1  Introduction  2

2  Background  and  Methods  6

3  Empirical  Results  and  Basic  Power  Laws  7

4  Charting  the  Infinite  Data  Limit  and  Overfitting  10

5  Scaling  Laws  with  Model  Size  and  Training  Time  12

6  Optimal  Allocation  of  the  Compute  Budget  14

7  Related  Work  18

8  Discussion  18

Appendices  20

A  Summary  of  Power  Laws  20

B  Empirical  Model  of  Compute-Efficient  Frontier  20

C  Caveats  22

D  Supplemental  Figures  23

# 1  Introduction

Language  provides  a  natural  domain  for  the  study  of  artificial  intelligence,  as  the  vast  maj ority  of  reason

’

ing  tasks  can  be  efficiently  expressed  and  evaluated  in  language,  and  the  world s  text  provides  a  wealth  of

data  for  unsupervised  learning  via  generative  modeling .  Deep  learning  has  recently  seen  rapid  progress  in  lan

guage  modeling,  with  state  of  the  art  models  [RNSS 1 8 ,  DCLT 1 8 ,  YDY+ 1 9,  LOG+ 1 9,  RSR+ 1 9]  approaching

human-level  performance  on  many  specific  tasks  [WPN+ 1 9] ,  including  the  composition  of  coherent  multi

paragraph  prompted  text  samples  [RWC+ 1 9] .

One  might  expect  language  modeling  performance  to  depend  on  model  architecture,  the  size  of  neural  models,

the  computing  power  used  to  train  them,  and  the  data  available  for  this  training  process .  In  this  work  we  will

empirically  investigate  the  dependence  of  language  modeling  loss  on  all  of  these  factors ,  focusing  on  the

Transformer  architecture  [VSP+ 1 7 ,  LSP+ 1 8] .  The  high  ceiling  and  low  floor  for  performance  on  language

tasks  allows  us  to  study  trends  over  more  than  seven  orders  of  magnitude  in  scale.

Throughout  we  will  observe  precise  power-law  scalings  for  performance  as  a  function  of  training  time,  con

text  length,  dataset  size,  model  size,  and  compute  budget.

1.1  Summary

Our  key  findings  for  Transformer  language  models  are  are  as  follows :

2Here  we  display  predicted  compute  when  using  a  sufficiently  small  batch  size.  See  Figure   1 3  for  comparison  to  the

purely  empirical  data.

2

s

s

o

L

t

s

e

T

Com pute 
 Dataset Size 
 Parameters

PF-days , non -em bedd i ng
 tokens
 non -em bedd i ng

Figure  1  Language  modeling  performance  improves  smoothly  as  we  increase  the  model  size,  datasetset

size,  and  amount  of  compute2  used  for  training .  For  optimal  performance  all  three  factors  must  be  scaled

up  in  tandem.  Empirical  performance  has  a  power-law  relationship  with  each  individual  factor  when  not

bottlenecked  by  the  other  two .

Performance  depends  strongly  on  scale,  weakly  on  model  shape:  Model  performance  depends  most

strongly  on  scale,  which  consists  of  three  factors :  the  number  of  model  parameters  N  (excluding  embed

dings) ,  the  size  of  the  dataset  D ,  and  the  amount  of  compute  C  used  for  training .  Within  reasonable  limits ,

performance  depends  very  weakly  on  other  architectural  hyperparameters  such  as  depth  vs .  width.  (Section

3 )

Smooth  power  laws :  Performance  has  a  power-law  relationship  with  each  of  the  three  scale  factors

N,  D ,  C  when  not  bottlenecked  by  the  other  two,  with  trends  spanning  more  than  six  orders  of  magnitude

(see  Figure   1 ) .  We  observe  no  signs  of  deviation  from  these  trends  on  the  upper  end,  though  performance

must  flatten  out  eventually  before  reaching  zero  loss .  (Section  3 )

Universality  of  overfitting:  Performance  improves  predictably  as  long  as  we  scale  up  N  and  D  in  tandem,

but  enters  a  regime  of  diminishing  returns  if  either  N  or  D  is  held  fixed  while  the  other  increases .  The

performance  penalty  depends  predictably  on  the  ratio  N0 . 74 /D ,  meaning  that  every  time  we  increase  the

model  size  8x,  we  only  need  to  increase  the  data  by  roughly  5x  to  avoid  a  penalty.  (Section  4)

Universality  of  training:  Training  curves  follow  predictable  power-laws  whose  parameters  are  roughly

independent  of  the  model  size.  By  extrapolating  the  early  part  of  a  training  curve,  we  can  roughly  predict  the

loss  that  would  be  achieved  if  we  trained  for  much  longer.  (Section  5)

Transfer  improves  with  test  performance:  When  we  evaluate  models  on  text  with  a  different  distribution

than  they  were  trained  on,  the  results  are  strongly  correlated  to  those  on  the  training  validation  set  with

a  roughly  constant  offset  in  the  los s –   in  other  words ,  transfer  to  a  different  distribution  incurs  a  constant

penalty  but  otherwise  improves  roughly  in  line  with  performance  on  the  training  set.  (Section  3 . 2. 2)

Sample  efficiency:  Large  models  are  more  sample-efficient  than  small  models,  reaching  the  same  level  of

performance  with  fewer  optimization  steps  (Figure  2)  and  using  fewer  data  points  (Figure  4) .

Convergence  is  inefficient:  When  working  within  a  fixed  compute  budget  C  but  without  any  other  restric

tions  on  the  model  size  N  or  available  data  D ,  we  attain  optimal  performance  by  training  very  large  models

and  stopping  significantly  short  of  convergence  (see  Figure  3) .  Maximally  compute-efficient  training  would

therefore  be  far  more  sample  efficient  than  one  might  expect  based  on  training  small  models  to  convergence,

with  data  requirements  growing  very  slowly  as  D  ∼  C
 0 . 27  with  training  compute.  (Section  6)

Optimal  batch  size :  The  ideal  batch  size  for  training  these  models  is  roughly  a  power  of  the  loss  only,

and  continues  to  be  determinable  by  measuring  the  gradient  noise  scale  [MKAT 1 8] ;  it  is  roughly   1 -2  million

tokens  at  convergence  for  the  largest  models  we  can  train.  (Section  5 . 1 )

Taken  together,  these  results  show  that  language  modeling  performance  improves  smoothly  and  predictably

as  we  appropriately  scale  up  model  size,  data,  and  compute.  We  expect  that  larger  language  models  will

perform  better  and  be  more  sample  efficient  than  current  models .

3

Larger models require fewer samples
 The optimal model size grows smoothly

to reach the same performance
 with the loss target and compute budget

Li n e co l o r i n d i cates

Test Loss
 1 0
 1 0
 n u m ber of parameters

8
 8

1 03 Params

1 03  1 06  1 09

6
 6
 -

9  Com pute efficient

1 0 Params
 trai n i n g sto ps far

short of convergence

4
 4

1 0 7  1 09  1 0 1 1
 1 0 -9  1 0 -6  1 0 -3  1 00

Tokens Processed  Compute (PF-days)

# Figure  2  We  show  a  series  of  language  model  training  runs ,  with  models  ranging  in  size  from  1 03
 to  1 09

# parameters  (excluding  embeddings) .

M i n i m u m serial steps 
 s
 Data req u i rements

i ncreases neg l ig i bly
 lS  tep g row relatively slowly

ria

xS  e ize

<10 chS

Bat

00x

1

e
 O pti mal mod el size

del Siz i ncreases very q u ickly

0x Mo

00,00

>1 ,0

# Figure  3  As  more  compute  becomes  available,  we  can  choose  how  much  to  allocate  towards  training  larger

# models ,  using  larger  batches ,  and  training  for  more  steps .  We  illustrate  this  for  a  billion-fold  increase  in

# compute.  For  optimally  compute-efficient  training,  most  of  the  increase  should  go  towards  increased  model

# size.  A  relatively  small  increase  in  data  is  needed  to  avoid  reuse.  Of  the  increase  in  data,  most  can  be  used  to

# increase  parallelism  through  larger  batch  sizes ,  with  only  a  very  small  increase  in  serial  training  time  required.

# 1.2  Summary  of  Scaling  Laws

# The  test  loss  of  a  Transformer  trained  to  autoregressively  model  language  can  be  predicted  using  a  power-law

# when  performance  is  limited  by  only  either  the  number  of  non-embedding  parameters  N,  the  dataset  size  D ,

# or  the  optimally  allocated  compute  budget  Cmin  (see  Figure   1 ) :

# 1 .  For  models  with  a  limited  number  of  parameters,  trained  to  convergence  on  sufficiently  large

## datasets :

## L ( N)  =   ( Nc /N) 
α N  ;  αN  ∼  0 . 0 76 ,  Nc  ∼  8 . 8  ×   1 0 1 3  (non-embedding  parameters)  ( 1 . 1 )

# 2 .  For  large  models  trained  with  a  limited  dataset  with  early  stopping :

## L ( D )  =   ( Dc / D ) 
α D  ;  α D  ∼  0 . 09 5 ,  Dc  ∼  5 . 4  ×   1 0 1 3  (tokens)  ( 1 . 2)

# 3 .  When  training  with  a  limited  amount  of  compute,  a  sufficiently  large  dataset,  an  optimally- sized

# model,  and  a  sufficiently  small  batch  size  (making  optimal3  use  of  compute) :

### m i n
 α
mC
 i n
 m i n
 m i n
 8
 -

## L ( Cmin )  =    C
c  / Cmin  ;  α
C  ∼  0 . 0 5 0 ,  Cc  ∼  3 . 1  ×   1 0 (PF days)  ( 1 . 3 )

###### 3 We  also  observe  an  empirical  power-law  trend  with  the  training  compute  C  (Figure   1 )  while  training  at  fixed  batch

###### size,  but  it  is  the  trend  with  Cmin  that  should  be  used  to  make  predictions .  They  are  related  by  equation  (5 . 5) .

# 4

# Loss vs Model and Dataset Size 
 Loss vs Model Size and Training Steps

4 . 4 
 )d

4 . 5 
 e

4 . 0 
 Params 
 4 . 0 
 1 0 8 b
 me

7 0 8 M 
 3 . 6 
 n-

ss 3 
. 5 
 3 0 2 M 
 ss 
 (on

Lo 83M5 M 
 
 Lo 3 . 2 
 1 0 7 
 sr

3 . 0 
 2 5M 
 tee

3 9 3 . 2 K 
 2 . 8 
 m

1 0 6 
 ara

2 . 5 
 2 . 4 
 P

1 0 7 
   1 0 8 
   1 0 9 
   1 0 1 0
 
 1 0 4 
   1 0 5

## Tokens in Dataset 
 E stimated S  m i n

Figure  4  Left:  The  early- stopped  test  loss  L ( N,  D )   varies  predictably  with  the  dataset  size  D  and  model

size  N  according  to  Equation  ( 1 . 5) .  Right:  After  an  initial  transient  period,  learning  curves  for  all  model

sizes  N  can  be  fit  with  Equation  ( 1 . 6) ,  which  is  parameterized  in  terms  of  Smin ,  the  number  of  steps  when

training  at  large  batch  size  (details  in  S ection  5 . 1 ) .

These  relations  hold  across  eight  orders  of  magnitude  in  Cmin ,  six  orders  of  magnitude  in  N,  and  over  two

orders  of  magnitude  in  D .  They  depend  very  weakly  on  model  shape  and  other  Transformer  hyperparameters

(depth,  width,  number  of  self-attention  heads) ,  with  specific  numerical  values  associated  with  the  Webtext2

training  set  [RWC+ 1 9] .  The  power  laws  αN ,  αD ,  αmC  in
 specify  the  degree  of  performance  improvement

expected  as  we  scale  up  N,  D ,  or  Cmin ;  for  example,  doubling  the  number  of  parameters  yields  a  loss  that

is  smaller  by  a  factor  2
− α N  =  0 . 9 5 .  The  precise  numerical  values  of  Nc ,  Ccm
 in
 ,  and  Dc  depend  on  the

vocabulary  size  and  tokenization  and  hence  do  not  have  a  fundamental  meaning.

The  critical  batch  size,  which  determines  the  speed/efficiency  tradeoff  for  data  parallelism  ( [MKAT 1 8] ) ,  also

roughly  obeys  a  power  law  in  L :

Bcrit  ( L )  =  B∗
 ,  B∗  ∼  2  ·   1 0 8
 tokens ,  α B  ∼  0 . 2 1   ( 1 . 4)

L 1 / α B

Equation  ( 1 . 1 )  and  ( 1 . 2)  together  suggest  that  as  we  increase  the  model  size,  we  should  increase  the  dataset

α N

size  sublinearly  according  to  D  ∝  N
 α D  ∼  N0 . 74 .  In  fact,  we  find  that  there  is  a  single  equation  combining

( 1 . 1 )  and  ( 1 . 2)  that  governs  the  simultaneous  dependence  on  N  and  D  and  governs  the  degree  of  overfitting :

α N
 α D

Nc
 α D
 Dc

" #

 N
  D

L ( N,  D )  =  +
 ( 1 . 5 )

with  fits  pictured  on  the  left  in  figure  4 .  We  conj ecture  that  this  functional  form  may  also  parameterize  the

trained  log-likelihood  for  other  generative  modeling  tasks .

When  training  a  given  model  for  a  finite  number  of  parameter  update  steps  S  in  the  infinite  data  limit,  after

an  initial  transient  period,  the  learning  curves  can  be  accurately  fit  by  (see  the  right  of  figure  4)

α N
 
 α S

Nc
 Sc

 N
   Smin ( S)
 

L ( N,  S )  =  +
 ( 1 . 6)

where  Sc  ≈  2 . 1  ×   1 03
 and  αS  ≈  0 . 76 ,  and  Smin ( S)   is  the  minimum  possible  number  of  optimization  steps

(parameter  updates)  estimated  using  Equation  (5 .4) .

When  training  within  a  fixed  compute  budget  C,  but  with  no  other  constraints,  Equation  ( 1 . 6)  leads  to  the

prediction  that  the  optimal  model  size  N,  optimal  batch  size  B ,  optimal  number  of  steps  S,  and  dataset  size

D  should  grow  as

α
mC  i n
 / α N  α
mC  i n
 / α B  α
mC  i n
 / α S  = ·

N  ∝  C
 ,  B  ∝  C
 ,  S  ∝  C
 ,  D    B    S  ( 1 . 7 )

with

α
mC  in
 =   1 /  ( 1 / α S  +   1 / α B  +   1 / α N  )   ( 1 . 8 )

which  closely  matches  the  empirically  optimal  results  N  ∝  C
m0 .i7n3  
 ,  B  ∝  C
m0 .i2n4  
 ,  and  S  ∝  C
m0 .i0n3  
 .  As  the

computational  budget  C  increases,  it  should  be  spent  primarily  on  larger  models,  without  dramatic  increases

in  training  time  or  dataset  size  (see  Figure  3 ) .  This  also  implies  that  as  models  grow  larger,  they  become

increasingly  sample  efficient.  In  practice,  researchers  typically  train  smaller  models  for  longer  than  would

5

be  maximally  compute-efficient  because  of  hardware  constraints .  Optimal  performance  depends  on  total

compute  as  a  power  law  (see  Equation  ( 1 . 3 )) .

We  provide  some  basic  theoretical  motivation  for  Equation  ( 1 . 5) ,  an  analysis  of  learning  curve  fits  and  their

implications  for  training  time,  and  a  breakdown  of  our  results  per  token.  We  also  make  some  brief  compar

isons  to  LSTMs  and  recurrent  Transformers  [DGV+ 1 8] .

1 .3  Notation

We  use  the  following  notation:

•  L  –  the  cros s  entropy  los s  in  nats .  Typically  it  will  be  averaged  over  the  tokens  in  a  context,  but  in

some  cases  we  report  the  loss  for  specific  tokens  within  the  context.

•  N  –  the  number  of  model  parameters,  excluding  all  vocabulary  and p  ositional  embeddings

•  C  ≈  6NB S  –  an  estimate  of  the  total  non-embedding  training  compute,  where  B  is  the  batch  size,

and  S  is  the  number  of  training  steps  (ie  parameter  updates) .  We  quote  numerical  values  in  PF-days,

where  one  PF-day  =  1 0 1 5  ×   24  ×   3600  =  8 . 64  ×   1 0 1 9  floating  point  operations .

•  D  –  the  dataset  size  in  tokens

•  Bcrit  –  the  critical  batch  size  [MKAT 1 8] ,  defined  and  discussed  in  Section  5 . 1 .  Training  at  the

critical  batch  size  provides  a  roughly  optimal  compromise  between  time  and  compute  efficiency.

•  Cmin  –  an  estimate  of  the  minimum  amount  of  non-embedding  compute  to  reach  a  given  value  of

the  los s .  This  is  the  training  compute  that  would  be  used  if  the  model  were  trained  at  a  batch  size

much  les s  than  the  critical  batch  size .

•  Smin  –  an  estimate  of  the  minimal  number  of  training  steps  needed  to  reach  a  given  value  of  the  loss .

This  is  also  the  number  of  training  steps  that  would  be  used  if  the  model  were  trained  at  a  batch  size

much  greater  than  the  critical  batch  size.

•  αX  –  power-law  exponents  for  the  scaling  of  the  loss  as  L (X )   ∝  1 /X αX  where  X  can  be  any  of

N,  D ,  C,  S,  B ,  Cmin
 .

# 2  Background  and  Methods

We  train  language  models  on  WebText2,  an  extended  version  of  the  WebText  [RWC+ 1 9]  dataset,  tokenized

using  byte-pair  encoding  [SHB 1 5]  with  a  vocabulary  size  nvocab  =  5025 7.  We  optimize  the  autoregres

sive  log-likelihood  (i . e.  cross-entropy  loss)  averaged  over  a   1 024-token  context,  which  is  also  our  principal

performance  metric .  We  record  the  loss  on  the  WebText2  test  distribution  and  on  a  selection  of  other  text

distributions .  We  primarily  train  decoder-only  [LSP+ 1 8 ,  RNS S 1 8]  Transformer  [VSP+ 1 7]  models,  though

we  also  train  LSTM  models  and  Universal  Transformers  [DGV+ 1 8]  for  comparison.

2.1  Parameter  and  Compute  Scaling  of  Transformers

We  parameterize  the  Transformer  architecture  using  hyperparameters  nlayer  (number  of  layers) ,  dmodel  (di

mension  of  the  residual  stream) ,  dff  (dimension  of  the  intermediate  feed-forward  layer) ,  dattn  (dimension  of

the  attention  output) ,  and  nheads  (number  of  attention  heads  per  layer) .  We  include  nctx  tokens  in  the  input

context,  with  nctx  =  1 024  except  where  otherwise  noted.

We  use  N  to  denote  the  model  size,  which  we  define  as  the  number  of  non-embedding  parameters

N  ≈  2dmodel nlayer  ( 2dattn  +  dff )

=   1 2nlayer d
m2
 odel  with  the  standard  dattn  =  dff /4  =  dmodel  (2 . 1 )

where  we  have  excluded  biases  and  other  sub-leading  terms .  Our  models  also  have  nvocab dmodel  parameters

in  an  embedding  matrix,  and  use  nctx dmodel  parameters  for  positional  embeddings,  but  we  do  not  include

‘ ’

these  when  discussing  the   model  size   N ;  we  will  see  that  this  produces  significantly  cleaner  scaling  laws .

Evaluating  a  forward  pass  of  the  Transformer  involves  roughly

Cforward  ≈  2N  +  2nlayer nctx dmodel  (2. 2)

add-multiply  operations,  where  the  factor  of  two  comes  from  the  multiply-accumulate  operation  used  in

matrix  multiplication.  A  more  detailed  per-operation  parameter  and  compute  count  is  included  in  Table   1 .

6

Operation  Parameters  FLOPs  per  Token

Embed  (nvocab  +  nctx )   dmodel  4dmodel

Attention:  QKV  nlayer dmodel 3dattn  2nlayer dmodel 3dattn

Attention:  Mask  —  2nlayer nctx dattn

Attention:  Proj ect  nlayer dattn dmodel  2nlayer dattn dembd

Feedforward  nlayer 2dmodel dff  2nlayer 2dmodel dff

De-embed  —  2dmodelnvocab

Total  (Non-Embedding)  N  =  2dmodel nlayer  ( 2dattn  +  dff )  Cforward  =  2N  +  2nlayer nctx dattn

Table  1  Parameter  counts  and  compute  (forward  pass)  estimates  for  a  Transformer  model.  Sub-leading

terms  such  as  nonlinearities ,  biases ,  and  layer  normalization  are  omitted.

For  contexts  and  models  with  dmodel  >  nctx / 1 2 ,  the  context-dependent  computational  cost  per  token  is  a

relatively  small  fraction  of  the  total  compute.  Since  we  primarily  study  models  where  dmodel    nctx / 1 2 ,

we  do  not  include  context-dependent  terms  in  our  training  compute  estimate.  Accounting  for  the  backwards

pass  (approximately  twice  the  compute  as  the  forwards  pass) ,  we  then  define  the  estimated  non-embedding

compute  as  C  ≈  6N  floating  point  operators  per  training  token.

2.2  Training  Procedures

Unless  otherwise  noted,  we  train  models  with  the  Adam  optimizer  [KB 1 4]  for  a  fixed  2 . 5  ×   1 05
 steps  with

a  batch  size  of  5 1 2  sequences  of  1 024  tokens .  Due  to  memory  constraints ,  our  largest  models  (more  than

1 B  parameters)  were  trained  with  Adafactor  [S S 1 8] .  We  experimented  with  a  variety  of  learning  rates  and

schedules,  as  discussed  in  Appendix  D . 6 .  We  found  that  results  at  convergence  were  largely  independent  of

learning  rate  schedule.  Unless  otherwise  noted,  all  training  runs  included  in  our  data  used  a  learning  rate

schedule  with  a  3000  step  linear  warmup  followed  by  a  cosine  decay  to  zero .

2.3  Datasets

We  train  our  models  on  an  extended  version  of  the  WebText  dataset  described  in  [RWC+ 1 9] .  The  original

WebText  dataset  was  a  web  scrape  of  outbound  links  from  Reddit  through  December  20 1 7  which  received  at

least  3  karma.  In  the  second  version,  WebText2,  we  added  outbound  Reddit  links  from  the  period  of  January

to  October  20 1 8 ,  also  with  a  minimum  of  3  karma.  The  karma  threshold  served  as  a  heuristic  for  whether

people  found  the  link  interesting  or  useful.  The  text  of  the  new  links  was  extracted  with  the  Newspaper3k

python  library.  In  total,  the  dataset  consists  of  20 . 3M  documents  containing  96  GB  of  text  and  1 . 62  ×   1 0 1 0

words  (as  defined  by  wc) .  We  then  apply  the  reversible  tokenizer  described  in  [RWC+ 1 9] ,  which  yields

2 . 2 9  ×   1 0 1 0  tokens .  We  reserve  6 . 6  ×   1 08  of  these  tokens  for  use  as  a  test  set,  and  we  also  test  on  similarly

prepared  samples  of  Books  Corpus  [ZKZ+ 1 5] ,  Common  Crawl  [Fou] ,  English  Wikipedia,  and  a  collection

of  publicly-available  Internet  B ooks .

# 3  Empirical  Results  and  Basic  Power  Laws

To  characterize  language  model  scaling  we  train  a  wide  variety  of  models,  varying  a  number  of  factors

including :

•  Model  size  (ranging  in  size  from  768  to   1 . 5  billion  non-embedding  parameters)

•  Dataset  size  (ranging  from  22  million  to  23  billion  tokens)

•  Shape  (including  depth,  width,  attention  heads,  and  feed-forward  dimension)

•  Context  length  ( 1 024  for  most  runs,  though  we  also  experiment  with  shorter  contexts)

•  B atch  size  (2
1 9  for  most  runs ,  but  we  also  vary  it  to  measure  the  critical  batch  size)

7

1 0 %

e
 8 %

s

aer 6%
 A wide range of arch itectu res

cn achieve simi lar performance

I  4 %
 22 % add itional com pute

ss com pensates for 1 % loss i ncrease

o 2 %

L

0 %

Feed- Forward Ratio  (dff / dmodel) 
 Attention Head Di mension (dmodel / nhead)

50M Parameters  Aspect Ratio  (dmodel / nlayer)  25 M Parameters

Figure  5  Performance  depends  very  mildly  on  model  shape  when  the  total  number  of  non-embedding

parameters  N  is  held  fixed.  The  loss  varies  only  a  few  percent  over  a  wide  range  of  shapes .  Small  differences

in  parameter  counts  are  compensated  for  by  using  the  fit  to  L ( N)   as  a  baseline.  Aspect  ratio  in  particular  can

vary  by  a  factor  of  40  while  only  slightly  impacting  performance ;  an  (nlayer ,  dmodel )  =   ( 6 ,  4288 )   reaches  a

loss  within  3 %  of  the  (48 ,  1 600 )   model  used  in  [RWC+ 1 9] .

7 
 7

6 
 6

5 
 5

ss 
 ss

## o 0 Layer 
 o

L  4 
 L  4

## ts 1  Layer 
 ts 1  Layer

## Te 2 Layers 
 Te 2 Layers

3 
 3 Layers 
 3 
 3 Layers

## 6 Layers 
 6 Layers

> 6   L  ayers 
 > 6   L  ayers

2 
 1 0 6 
   1 0 7 
   1 0 8 
   1 0 9 
 
 2 
 1 0 3 
   1 0 4 
   1 0 5 
   1 0 6 
   1 0 7 
   1 0 8 
   1 0 9

# Parameters (with embedding) 
 Parameters (non-embedding)

Figure  6  Left:  When  we  include  embedding  parameters,  performance  appears  to  depend  strongly  on  the

number  of  layers  in  addition  to  the  number  of  parameters .  Right:  When  we  exclude  embedding  parameters,

the  performance  of  models  with  different  depths  converge  to  a  single  trend.  Only  models  with  fewer  than  2

layers  or  with  extreme  depth-to-width  ratios  deviate  significantly  from  the  trend.

In  this  section  we  will  display  data  along  with  empirically-motivated  fits ,  deferring  theoretical  analysis  to

later  sections .

3.1  Approximate  Transformer  Shape  and  Hyperparameter  Independence

Transformer  performance  depends  very  weakly  on  the  shape  parameters  nlayer ,  nheads ,  and  dff  when  we  hold

the  total  non-embedding  parameter  count  N  fixed.  To  establish  these  results  we  trained  models  with  fixed

size  while  varying  a  single  hyperparameter.  This  was  simplest  for  the  case  of  nheads .  When  varying  nlayer ,

we  simultaneously  varied  dmodel  while  keeping  N  ≈  1 2nlayer d
m2
 odel  fixed.  Similarly,  to  vary  dff  at  fixed

model  size  we  also  simultaneously  varied  the  dmodel  parameter,  as  required  by  the  parameter  counts  in  Table

1 .  Independence  of  nlayers  would  follow  if  deeper  Transformers  effectively  behave  as  ensembles  of  shallower

models,  as  has  been  suggested  for  ResNets  [VWB 1 6] .  The  results  are  shown  in  Figure  5 .

3.2  Performance  with  Non-Embedding  Parameter  Count  N

In  Figure  6  we  display  the  performance  of  a  wide  variety  of  models,  ranging  from  small  models  with  shape

(nlayer ,  dmodel )  =   ( 2 ,  1 28 )   through  billion-parameter  models ,  ranging  in  shape  from  ( 6 ,  4288 )   through

( 207 ,  768 ) .  Here  we  have  trained  to  near  convergence  on  the  full  WebText2  dataset  and  observe  no  over

fitting  (except  possibly  for  the  very  largest  models) .

As  shown  in  Figure   1 ,  we  find  a  steady  trend  with  non-embedding  parameter  count  N,  which  can  be  fit  to  the

first  term  of  Equation  ( 1 . 5 ) ,  so  that

α N

Nc

 N
 

L ( N )   ≈
 (3 . 1 )

8

Transformers asymptotically outperform LSTMs 
 LSTM  plateaus after < 1 00 tokens

d ue to i m proved use of long contexts
 Transformer i m proves th roug h the whole context

Test Loss  5 . 4
 Per-token

4 . 8

Test Loss
 6

4 . 2
 LSTM s
 4

Parameters
:

3 . 6
 400 K

1  Laye 
r 5
 400 K

2 Layers
 2 M

3 . 0
 Transfo rm ers
 4 Layers
 3 M

3
 2 00 M

2 . 4
 300 M

2

1 05  1 06  1 0 7  1 08  1 09
 1 0 1  1 02  1 03

Parameters (non-em bedd i ng)
 Token I ndex i n Context

Figure  7

To  observe  these  trends  it  is  crucial  to  study  performance  as  a  function  of  N ;  if  we  instead  use  the  total

parameter  count  (including  the  embedding  parameters)  the  trend  is  somewhat  obscured  (see  Figure  6) .  This

suggests  that  the  embedding  matrix  can  be  made  smaller  without  impacting  performance,  as  has  been  seen  in

recent  work  [LCG+ 1 9] .

Although  these  models  have  been  trained  on  the  WebText2  dataset,  their  test  loss  on  a  variety  of  other  datasets

is  also  a  power-law  in  N  with  nearly  identical  power,  as  shown  in  Figure  8 .

3.2.1  Comparing  to  LSTMs  and  Universal  Transformers

In  Figure  7  we  compare  LSTM  and  Transformer  performance  as  a  function  of  non-embedding  parameter

count  N.  The  LSTMs  were  trained  with  the  same  dataset  and  context  length.  We  see  from  these  figures

that  the  LSTMs  perform  as  well  as  Transformers  for  tokens  appearing  early  in  the  context,  but  cannot  match

the  Transformer  performance  for  later  tokens .  We  present  power-law  relationships  between  performance  and

context  position  Appendix  D . 5 ,  where  increasingly  large  powers  for  larger  models  suggest  improved  ability

to  quickly  recognize  patterns .

We  also  compare  the  performance  of  standard  Transformers  to  recurrent  Transformers  [DGV+ 1 8]  in  Figure

1 7  in  the  appendix.  These  models  re-use  parameters ,  and  so  perform  slightly  better  as  a  function  of  N,  at  the

cost  of  additional  compute  per-parameter.

3.2.2  Generalization  Among  Data  Distributions

We  have  also  tested  our  models  on  a  set  of  additional  text  data  distributions .  The  test  los s  on  these  datasets

as  a  function  of  model  size  is  shown  in  Figure  8 ;  in  all  cases  the  models  were  trained  only  on  the  WebText2

dataset.  We  see  that  the  loss  on  these  other  data  distributions  improves  smoothly  with  model  size,  in  direct

parallel  with  the  improvement  on  WebText2.  We  find  that  generalization  depends  almost  exclusively  on  the

in-distribution  validation  loss ,  and  does  not  depend  on  the  duration  of  training  or  proximity  to  convergence.

We  also  observe  no  dependence  on  model  depth  (see  Appendix  D . 8) .

3.3  Performance  with  Dataset  Size  and  Compute

We  display  empirical  trends  for  the  test  loss  as  a  function  of  dataset  size  D  (in  tokens)  and  training  compute

C  in  Figure   1 .

For  the  trend  with  D  we  trained  a  model  with  (nlayer ,  nembd )  =   ( 36 ,  1 280 )   on  fixed  subsets  of  the  WebText2

dataset.  We  stopped  training  once  the  test  los s  ceased  to  decrease.  We  see  that  the  resulting  test  los ses  can  be

fit  with  simple  power-law

α D

Dc

 D
 

L ( D )   ≈
 (3 . 2)

in  the  dataset  size .  The  data  and  fit  appear  in  Figure   1 .

The  total  amount  of  non-embedding  compute  used  during  training  can  be  estimated  as  C  =  6NBS,  where

B  is  the  batch  size,  S  is  the  number  of  parameter  updates ,  and  the  factor  of  6  accounts  for  the  forward  and

backward  passes .  Thus  for  a  given  value  of  C  we  can  scan  over  all  models  with  various  N  to  find  the  model

9

7 
 5 . 0

WebText2 (Test) 
 n 
 Books during training

6 
 Internet Books 
 ito 4 . 5 
 Wikipedia during training

Books 
 u Books at convergence

5 
 Wikipedia 
 bir 4 . 0 
 Wikipedia at convergence

s 
 Common Crawl 
 ts

# s i

## Lo D  3 . 5

t  4 
 re

# se ht

T O 3 . 0

3 
 no

# s

so 2 . 5

# L

1 0 4 
   1 0 5 
   1 0 6 
   1 0 7 
   1 0 8 
   1 0 9 
 
 5 . 0   4 . 5   4 . 0   3 . 5   3 . 0   2 . 5

# Parameters (non-embedding) 
 Test Loss on Training Distribution

Figure  8  Left:  Generalization  performance  to  other  data  distributions  improves  smoothly  with  model  size,

with  only  a  small  and  very  slowly  growing  offset  from  the  WebText2  training  distribution.  Right:  Gener

alization  performance  depends  only  on  training  distribution  performance,  and  not  on  the  phase  of  training .

We  compare  generalization  of  converged  models  (points)  to  that  of  a  single  large  model  (dashed  curves)  as  it

trains .

with  the  best  performance  on  step  S  =
 6BC
S  .  Note  that  in  these  results  the  batch  size  B  remains fi  xed f  or

all  models,  which  means  that  these  empirical  results  are  not  truly  optimal.  We  will  account  for  this  in  later

sections  using  an  adjusted  Cmin  to  produce  cleaner  trends .

The  result  appears  as  the  heavy  black  line  on  the  left-hand  plot  in  Figure   1 .  It  can  be  fit  with

α C

Cc

 C
 

L ( C )   ≈
 (3 . 3 )

The  figure  also  includes  images  of  individual  learning  curves  to  clarify  when  individual  models  are  optimal.

We  will  study  the  optimal  allocation  of  compute  more  closely  later  on.  The  data  strongly  suggests  that  sample

efficiency  improves  with  model  size,  and  we  also  illustrate  this  directly  in  Figure   1 9  in  the  appendix.

## 4  Charting  the  Infinite  Data  Limit  and  Overfitting

In  Section  3  we  found  a  number  of  basic  scaling  laws  for  language  modeling  performance.  Here  we  will

study  the  performance  of  a  model  of  size  N  trained  on  a  dataset  with  D  tokens  while  varying  N  and  D

simultaneously.  We  will  empirically  demonstrate  that  the  optimally  trained  test  loss  accords  with  the  scaling

law  of  Equation  ( 1 . 5) .  This  provides  guidance  on  how  much  data  we  would  need  to  train  models  of  increasing

size  while  keeping  overfitting  under  control.

4.1  Proposed  L (N,  D )  Equation

We  have  chosen  the  parameterization  ( 1 . 5)  (repeated  here  for  convenience) :

α N
 α D

Nc
 α D
 Dc

 N
  D

" #

L ( N,  D )  =  +
 (4 . 1 )

using  three  principles :

1 .  Changes  in  vocabulary  size  or  tokenization  are  expected  to  rescale  the  loss  by  an  overall  factor.  The

parameterization  of  L ( N,  D )   (and  all  models  of  the  loss)  must  naturally  allow  for  such  a  rescaling .

2.  Fixing  D  and  sending  N  →  ∞ ,  the  overall  loss  should  approach  L (D ) .  Conversely,  fixing  N  and

sending  D  →  ∞  the  loss  must  approach  L (N) .

3 .  L ( N,  D )   should  be  analytic  at  D  =  ∞ ,  so  that  it  has  a  series  expansion  in  1 / D  with  integer  powers .

Theoretical  support  for  this  principle  is  significantly  weaker  than  for  the  first  two .

Our  choice  of  L (N,  D )   satisfies  the  first  requirement  because  we  can  rescale  Nc ,  Dc  with  changes  in  the

vocabulary.  This  also  implies  that  the  values  of  Nc ,  Dc  have  no  fundamental  meaning .

1 0

# Data Size Bottleneck 
 Overfitting

0 . 5

4 . 5

## D ata S ize 
 0 . 4 
 D ata S ize

4 . 0 
 2 1 M 
 1 
 2 1 M

ss 
 4 3M 
 )   4 3M

Lo 3 . 5 
 8 6M 
 0 . 3 
 8 6M

t  1 7 2 M 
 =   1 7 2 M

se 344M 
 (D 0 . 2
 
 344M

T 3 . 0 
 68 8M 
 L/ 
 68 8M

1 . 4 B 
 L 
 1 . 4 B

2 2 . 0 B 
 0 . 1 
 2 2 . 0 B

2 . 5

1 0 6 
   1 0 7 
   1 0 8 
   1 0 9 
 
 0 . 0 
 1 0 4
   1 0 3
   1 0 2
   1 0 1

## Params (non-embed) 
 N N/    D/D

Figure  9  The  early- stopped  test  loss  L (N,  D )   depends  predictably  on  the  dataset  size  D  and  model  size  N

according  to  Equation  ( 1 . 5) .  Left:  For  large  D ,  performance  is  a  straight  power  law  in  N .  For  a  smaller  fixed

D ,  performance  stops  improving  as  N  increases  and  the  model  begins  to  overfit.  (The  reverse  is  also  true,

α N

see  Figure  4 . )  Right:  The  extent  of  overfitting  depends  predominantly  on  the  ratio  N
 α D  /D ,  as  predicted  in

equation  (4 . 3 ) .  The  line  is  our  fit  to  that  equation.

Since  we  stop  training  early  when  the  test  loss  ceases  to  improve  and  optimize  all  models  in  the  same  way,  we

expect  that  larger  models  should  always  perform  better  than  smaller  models .  But  with  fixed  finite  D ,  we  also

do  not  expect  any  model  to  be  capable  of  approaching  the  best  possible  loss  (ie  the  entropy  of  text) .  Similarly,

a  model  with  fixed  size  will  be  capacity-limited.  These  considerations  motivate  our  second  principle.  Note

that  knowledge  of  L (N)   at  infinite  D  and  L (D )   at  infinite  N  fully  determines  all  the  parameters  in  L (N,  D ) .

The  third  principle  is  more  speculative.  There  is  a  simple  and  general  reason  one  might  expect  overfitting

to  scale  ∝  1 / D  at  very  large  D .  Overfitting  should  be  related  to  the  variance  or  the  signal-to-noise  ratio

of  the  dataset  [AS 1 7] ,  and  this  scales  as  1 / D .  This  expectation  should  hold  for  any  smooth  loss  function,

since  we  expect  to  be  able  to  expand  the  loss  about  the  D  →  ∞  limit.  However,  this  argument  assumes  that

1 / D  corrections  dominate  over  other  sources  of  variance,  such  as  the  finite  batch  size  and  other  limits  on  the

efficacy  of  optimization.  Without  empirical  confirmation,  we  would  not  be  very  confident  of  its  applicability.

Our  third  principle  explains  the  asymmetry  between  the  roles  of  N  and  D  in  Equation  ( 1 . 5) .  Very  similar

symmetric  expressions4
 are  possible,  but  they  would  not  have  a  1 /D  expansion  with  integer  powers,  and

would  require  the  introduction  of  an  additional  parameter.

In  any  case,  we  will  see  that  our  equation  for  L ( N,  D )   fits  the  data  well,  which  is  the  most  important j  ustifi

cation  for  our  L ( N,  D )   ansatz .

4.2  Results

We  regularize  all  our  models  with   1 0%  dropout,  and  by  tracking  test  loss  and  stopping  once  it  is  no  longer

decreasing .  The  results  are  displayed  in  Figure  9 ,  including  a  fit  to  the  four  parameters  αN  ,  αD ,  Nc ,  Dc  in

Equation  ( 1 . 5 ) :

Parameter  αN  αD  Nc  Dc

Value  0 . 0 76  0 . 1 0 3  6 . 4  ×   1 0 1 3  1 . 8  ×   1 0 1 3

Table  2  Fits  to  L ( N,  D )

We  obtain  an  excellent  fit,  with  the  exception  of  the  runs  where  the  dataset  has  been  reduced  by  a  factor  of

1 024,  to  about  2  ×   1 07
 tokens .  With  such  a  small  dataset,  an  epoch  consists  of  only  40  parameter  updates .

Perhaps  such  a  tiny  dataset  represents  a  different  regime  for  language  modeling,  as  overfitting  happens  very

early  in  training  (see  Figure   1 6) .  Also  note  that  the  parameters  differ  very  slightly  from  those  obtained  in

Section  3 ,  as  here  we  are  fitting  the  full  L ( N,  D )   rather  than j  ust  L ( N,  ∞ )   or  L ( ∞ ,  D ) .

To  chart  the  borderlands  of  the  infinite  data  limit,  we  can  directly  study  the  extent  of  overfitting .  For  all  but

the  largest  models,  we  see  no  sign  of  overfitting  when  training  with  the  full  22B  token  WebText2  dataset,

so  we  can  take  it  as  representative  of  D  =  ∞ .  Thus  we  can  compare  finite  D  to  the  infinite  data  limit  by

4
 =   Nc
 α N    D c
 α D
 β

For  example,  one  might  have  used  L ( N,  D )        N
  +
   D
   ,  but  this  does  not  have  a  1 / D  expansion.

1 1

)

## s

## n

ke 1 0 6

## o

## (T

# Critical Batch Size vs . Performance

## e

iz 5

S 1 0

## h

## tc

Ba 4 
 E mpirical   Bcrit,   N =  3  M

la  1 0 
 E mpirical   Bcrit,   N =  8  5 M

itc B c rit   = 2  . 1 ×  1  0 8
 
 to ke n s   L 
 4 . 8

ir Noise Scale Measurement

C 1 0 3

1 0 1
   6 ×  1  0 0
 
 4 ×  1  0 0
   3 ×  1  0 0

### WebText2 Train Loss

Figure  10  The  critical  batch  size  Bcrit  follows  a  power  law  in  the  loss  as  performance  increase,  and  does

not  depend  directly  on  the  model  size.  We  find  that  the  critical  batch  size  approximately  doubles  for  every

1 3 %  decrease  in  loss .  Bcrit  is  measured  empirically  from  the  data  shown  in  Figure   1 8 ,  but  it  is  also  roughly

predicted  by  the  gradient  noise  scale,  as  in  [MKAT 1 8] .

defining

L (N,  D )
 −

δL ( N,  D )   ≡
   1   (4 . 2)

L (N,  ∞ )

and  studying  it  as  a  function  of  N,  D .  In  fact,  we  see  empirically  that  δL  depends  only  a  specific  combination

of  N  and  D ,  as  shown  in  Figure   1 6 .  This  follows  from  the  scaling  law  of  Equation  ( 1 . 5) ,  which  implies

α N
 α D

N
 α D  Dc

δL  ≈
 1  +  −   1   (4 . 3 )

!

 Nc
  D

Note  that  at  large  D  this  formula  also  has  a  series  expansion  in  powers  of  1 / D .

We  estimate  that  the  variation  in  the  loss  with  different  random  seeds  is  roughly  0 . 02 ,  which  means  that  to

avoid  overfitting  when  training  to  within  that  threshold  of  convergence  we  require

D  &  ( 5  ×   1 0 3
 )   N
 0 . 74  (4 . 4)

With  this  relation,  models  smaller  than  1 09  parameters  can  be  trained  with  minimal  overfitting  on  the  22B

token  WebText2  dataset,  but  our  largest  models  will  encounter  some  mild  overfitting.  More  generally,  this

relation  shows  that  dataset  size  may  grow  sub-linearly  in  model  size  while  avoiding  overfitting .  Note  however

that  this  does  not  typically  represent  maximally  compute-efficient  training.  We  should  also  emphasize  that

we  have  not  optimized  regularization  (eg  the  dropout  probability)  while  varying  dataset  and  model  size.

#### 5  Scaling  Laws  with  Model  Size  and  Training  Time

In  this  section  we  will  demonstrate  that  a  simple  scaling  law  provides  a  good  description  for  the  loss  as  a

function  of  model  size  N  and  training  time.  First  we  will  explain  how  to  use  the  results  of  [MKAT 1 8]  to

define  a  universal  training  step  Smin ,  which  accounts  for  the  fact  that  most  of  our  models  have  not  been

trained  at  an  optimal  batch  size.  Then  we  will  demonstrate  that  we  can  fit  the  model  size  and  training  time

dependence  of  the  loss  using  Equation  ( 1 . 6) .  Later  we  will  use  these  results  to  predict  the  optimal  allocation

of  training  compute  between  model  size  and  training  time,  and  then  confirm  that  prediction.

5. 1  Adjustment  for  Training  at  Bcrit (L)

A  simple  empirical  theory  for  the  batch  size  dependence  of  training  was  developed  in  [MKAT 1 8]  (see  also

[SLA+ 1 8 ,  ZLN+ 1 9] ) .  It  was  argued  that  there  is  a  critical  batch  size  Bcrit  for  training ;  for  B  up  to  Bcrit

the  batch  size  can  be  increased  with  very  minimal  degradation  in  compute-efficiency,  whereas  for  B  >  Bcrit

increases  in  B  result  in  diminishing  returns .  It  was  also  argued  that  the  gradient  noise  scale  provides  a  simple

1 2

prediction  for  Bcrit ,  and  that  neither  depends  directly  on  model  size  except  through  the  value  of  the  loss  that

has  been  attained.  These  results  can  be  used  to  predict  how  training  time  and  compute  will  vary  with  the

batch  size .  To  utilize  both  training  time  and  compute  as  effectively  as  pos sible,  it  is  best  to  train  with  a  batch

size  B  ≈  Bcrit .  Training  at  B    Bcrit  minimizes  the  number  of  training  steps,  while  B    Bcrit  minimizes

the  use  of  compute.

More  specifically,  it  was  demonstrated  that  for  a  wide  variety  of  neural  network  tasks,  the  number  of  training

steps  S  and  the  number  of  data  examples  processed  E  =  B S  satisfy  the  simple  relation

S
 E

−   1 
 −   1 
 =   1   ( 5 . 1 )

 Smin
   Emin
 

when  training  to  any  fixed  value  of  the  loss  L .  Here  Smin  is  the  minimum  number  of  steps  necessary  to  reach

L,  while  Emin  is  the  minimum  number  of  data  examples  that  must  be  processed.

We  demonstrate  the  relation  (5 . 1 )  for  Transformers  in  Figure   1 8  in  the  appendix.  This  relation  defines  the

critical  batch  size

Emin

Bcrit ( L )   ≡
 (5 . 2)

Smin

which  is  a  function  of  the  target  value  of  the  los s .  Training  at  the  critical  batch  size  makes  a  roughly  optimal

time/compute  tradeoff,  requiring  2 Smin  training  steps  and  processing  E  =  2Emin  data  examples .

In  Figure   1 0  we  have  plotted  the  critical  batch  size  and  gradient  noise  scale5
 as  a  function  of  training  los s  for

two  different  models .  We  see  that  Bcrit ( L)   is  independent  of  model  size,  and  only  depends  on  the  loss  L .  So

the  predictions  of  [MKAT 1 8]  continue  to  hold  for  Transformer  language  models .  The  critical  batch  size  can

be  fit  with  a  power-law  in  the  los s

B∗

Bcrit ( L )   ≈
 1 α (5 . 3 )

L / B

where  B∗  ≈  2  ×   1 08
 and  α B  ≈  0 . 2 1 .

We  have  chosen  this  parameterization  for  Bcrit (L)  because  as  the  loss  approaches  its  minimum  value  Lmin ,

the  gradient  noise  scale  is  expected  to  diverge,  and  we  expect  Bcrit  to  track  this  noise  scale.  We  do  not

know  Lmin ,  as  we  see  no  sign  that  our  models  are  approaching  it,  but  Lmin  >  0  since  the  entropy  of  natural

language  is  non-zero .  Since  apparently  Lmin  is  much  smaller  than  the  values  of  L  we  have  achieved,  we  used

a  parameterization  where  Bcrit  diverges  as  L  →  0 .

We  will  use  Bcrit ( L)   to  estimate  the  relation  between  the  number  of  training  steps  S  while  training  at  batch

size  B  =  2 1 9  tokens  and  the  number  of  training  steps  while  training  at  B    Bcrit .  This  is  simply

S

Smin ( S)   ≡
 (minimum  steps ,  at  B    Bcrit )   (5 .4)

1  +  Bcrit ( L) / B

for  any  given  target  value  L  for  the  los s .  This  also  defines  a  critical  value  of  the  compute  needed  to  train  to  L

with  a  model  of  size  N  if  we  were  to  train  at  B    Bcrit ( L ) .  This  is

C

Cmin ( C)  ≡
 (minimum  compute,  at  B    Bcrit )  (5 . 5)

1  +  B / Bcrit ( L )

where  C  =  6NBS  estimates  the  (non-embedding)  compute  used  at  batch  size  B .

5.2  Results  for  L (N,  Smin )  and  Performance  with  Model  Size  and  Compute

Now  we  will  use  Smin  defined  in  Equation  (5 .4)  to  obtain  a  simple  and  universal  fit  for  the  dependence  of  the

loss  on  model  size  and  training  time  in  the  infinite  data  limit.  We  will  fit  the  stable,  Adam-optimized  training

runs  using  Equation  ( 1 . 6) ,  repeated  here  for  convenience :

α N
 
 α S

Nc
 Sc

 N
   Smin  

L ( N,  Smin )  =  +
 (5 . 6)

for  the  loss .  We  include  all  training  steps  after  the  warmup  period  of  the  learning  rate  schedule,  and  find  a  fit

to  the  data  with  the  parameters :

5 Although  the  critical  batch  size  roughly  matches  the  gradient  noise  scale,  we  are  using  a  direct  measurements  of

Bcrit  from  Figures   1 8  and   1 0  for  all  our  later  analyses .

1 3

# Performance vs Compute Budget 
 Performance vs Steps

8 
 5 . 4

7 
 1 0 0 
 
 4 . 8

6 
 1 0 5

1 0 
1 
 4 . 2

ss 5
 
 ss 
 ss

o 1 0 2
 
 y o 3 . 6 
 sp

### Lt  4 
 da- Lt  te

se 1 0 3
 F
 se 3 . 0 
 S

## T P T

3 
 1 0 4
 
 1 0 4

5 
 2 . 4

1 0

2 
 1 0 4 
   1 0 6 
   1 0 8 
 
 1 0 6 
   1 0 7 
   1 0 8 
   1 0 9

## Parameters (non-embedding) 
 Parameters (non-embedding)

Figure  1 1  When  we  hold  either  total  compute  or  number  of  training  steps  fixed,  performance  follows

L (N,  S)   from  Equation  (5 . 6) .  Each  value  of  compute  budget  has  an  associated  optimal  model  size  that

maximizes  performance.  Mediocre  fits  at  small  S  are  unsurprising,  as  the  power-law  equation  for  the  learning

curves  breaks  down  very  early  in  training .

Parameter  αN  αS  Nc  Sc

Value  0 . 0 77  0 . 76  6 . 5  ×   1 0 1 3  2 . 1   ×   1 03

Table  3  Fits  to  L ( N,  S)

With  these  parameters,  we  obtain  the  learning  curve  fits  in  Figure  4 .  Though  the  fits  are  imperfect,  we  believe

they  are  quite  compelling  given  the  simplicity  of  Equation  (5 . 6) .

The  data  and  fits  can  be  visualized  in  a  different  and  more  interesting  way,  as  shown  in  Figure   1 1 .  There  we

study  the  test  loss  as  a  function  of  model  size  while  fixing  either  the  total  non-embedding  compute  C  used

in  training,  or  the  number  of  steps  S .  For  the  fits  we  use  Equation  (5 . 5)  and  (5 .4)  along  with  the  parameters

above  and  Equation  (5 . 6) .

The  power-law  dependence  of  the  loss  on  Smin  reflects  the  interplay  of  optimizer  dynamics  and  the  loss

landscape.  Since  the  fits  are  best  late  in  training,  when  the  loss  may  be  approximately  quadratic,  the  power

law  should  provide  information  about  the  spectrum  of  the  Hessian  of  the  loss .  Its  universality  suggests  that

the  Hessian  eigenvalue  density  is  roughly  independent  of  model  size.

5.3  Lower  Bound  on  Early  Stopping  Step

The  results  for  L ( N,  Smin )   can  be  used  to  derive  a  lower-bound  (and  rough  estimate)  of  the  step  at  which

early  stopping  should  occur  when  training  is  data  limited.  It  is  motivated  by  the  idea  that  finite  and  infinite  D

learning  curves  for  a  given  model  will  be  very  similar  until  we  reach  Smin  ≈  Sstop .  Thus  overfitting  should

be  proportional  to  the  correction  from  simply  ending  training  at  Sstop .  This  will  underestimate  Sstop ,  because

in  reality  the  test  loss  will  decrease  more  slowly  when  we  have  a  finite  D ,  and  therefore  we  will  require  more

training  steps  to  reach  the  optimal  test  los s  at  finite  D .  This  line  of  reasoning  leads  to  the  inequality

Sc

Sstop ( N,  D )   &
 1 α S
 (5 . 7 )

[L (N,  D )  −  L (N,  ∞ ) ] /

where  L (N,  ∞ )   is  the  converged  loss,  evaluated  with  infinite  available  data.  This  inequality  and  its  com

parison  to  the  empirical  data  is  displayed  in  Figure   1 6  in  the  appendix.  In  that  figure,  the  values  of  Sstop

and  L ( N,  D )   are  empirical  (though  Sstop  is  adjusted  to  mimic  training  at  B    Bcrit ) ,  while  L ( N,  ∞ )   is

computed  from  the  fit  to  L (N,  D )  evaluated  at  D  =  ∞ .

### 6  Optimal  Allocation  of  the  Compute  Budget

We  displayed  the  empirical  trend  of  performance  as  a  function  of  the  computation  used  during  training  in

the  top-right  of  Figure   1 .  However,  this  result  involved  training  at  a  fixed  batch  size  B ,  whereas  we  know

1 4

Models between 0.6x and 2.2x the

opti mal size can be trai ned with a

20% larger compute budget

Smal ler models req u i re

more steps to trai n , wh i le

larger models req u i re fewe
r

Ou r framework does not

captu re early trai n i ng dynam ics

Figure  12  Left:  Given  a  fixed  compute  budget,  a  particular  model  size  is  optimal,  though  somewhat  larger

or  smaller  models  can  be  trained  with  minimal  additional  compute.  Right:  Models  larger  than  the  compute

efficient  size  require  fewer  steps  to  train,  allowing  for  potentially  faster  training  if  sufficient  additional  paral

lelism  is  pos sible .  Note  that  this  equation  should  not  be  trusted  for  very  large  models ,  as  it  is  only  valid  in  the

power-law  region  of  the  learning  curve,  after  initial  transient  effects .

7 
 L =  ( C  m i n / 2 . 3   1 0 8 )
 
   0 . 0 5 0

6 
 L =  ( C  / 2 . 0   1 0 7 )
 
   0 . 0 5 7

5

ss

# o

L  4

# ts

# e

# T

3

120 
 
8   1 0 
6   1 0 
4   1 0 
2   1 0 0

# Compute (PF-days), non-embedding

Figure  13  When  adjusting  performance  to  simulate  training  far  below  the  critical  batch  size,  we  find  a

somewhat  altered  power  law  for  L ( Cmin )  when  compared  with  the  fully  empirical  results .  The  conspicuous

lump  at  1 0 − 5  PF-days  marks  the  transition  from   1 -layer  to  2-layer  networks ;  we  exclude   1 -layer  networks

in  the  power-law  fits .  It  is  the  L ( Cmin )   trend  that  we  expect  to  provide  a  reliable  extrapolation  for  larger

compute.

that  in  fact  we  could  train  more  efficiently6  by  training  at  the  batch  size  Bcrit  discus sed  in  Section  5 . 1 .

Large  and  small  values  of  the  loss  could  have  been  achieved  with  fewer  samples  or  fewer  steps,  respectively,

and  correcting  for  this  inefficiency  by  standardizing  to  the  critical  batch  size  results  in  cleaner  and  more

predictable  trends .

In  this  section  we  will  adj ust  for  this  oversight.  More  importantly,  we  will  use  the  results  of  Section  5

to  determine  the  optimal  allocation  of  compute  between  model  size  N  and  the  quantity  of  data  processed

during  training,  namely  2Bcrit Smin .  We  will  determine  this  allocation  both  empirically  and  theoretically,  by

using  the  equation  for  L (N,  Smin ) ,  and  we  will  demonstrate  that  these  methods  agree.

6.1  Optimal  Performance  and  Allocations

Let  us  first  study  the  loss  as  a  function  of  the  optimally  allocated  compute  from  Equation  (5 . 5) .  The  result  is

plotted  in  Figure   1 3 ,  along  with  a  power-law  fit.  We  see  that  as  compared  to  the  compute  plot  of  Figure   1 ,  the

new  fit  with  Cmin  is  somewhat  improved.

Given  L ( Cmin ) ,  it  is  natural  to  ask  for  the  optimal  model  size  N ( Cmin )   that  provides  the  minimal  loss  with  a

given  quantity  of  training  compute.  The  optimal  model  size  is  shown  in  Figure   1 4.  We  observe  that  N ( Cmin )

6 One  might  ask  why  we  did  not  simply  train  at  Bcrit  in  the  first  place.  The  reason  is  that  it  depends  not  only  on  the

model  but  also  on  the  target  value  of  the  los s  we  wish  to  achieve,  and  so  is  a  moving  target.

1 5

)

ign N =  ( 1  . 3   1 0 9 )
 
   Cm0 .
i7n3 
 
 S m i n (  adj u ste d )

dd N =  ( 1  . 6   1 0 9 )
 
   C 0 .
 8 8 
 1 5 0 0 0 
 S m i n =  ( 5  . 4   1 0 3 )
 
   Cm0 .
i0n3

be 1 0 7 
 
 S (  fixe d-b atch)

# m

e- s

## no p 1 0 0 0 0

(n 1 0 5 
 
 Ste

# s

# r

# te

e 5 0 0 0

ma 1 0 3

# r

# a

# P

1 0 
7   1 0 
5   1 0 
3   1 0 
1 
 0 
 1 0 
7   1 0 
5   1 0 
3   1 0 
1

# Compute (PF-days), non-embedding 
 Compute (PF-days), excluding embeddings

Figure  14  Left:  Each  value  of  the  compute  budget  Cmin  has  an  associated  optimal  model  size  N .  Optimal

model  size  grows  very  rapidly  with  Cmin ,  increasing  by  5x  for  each   1 0x  increase  in  compute.  The  number

of  data  examples  processed  makes  up  the  remainder  of  the  increase,  growing  relatively  modestly  by  only  2x.

Right:  The  batch-adjusted  number  of  optimization  steps  also  grows  very  slowly,  if  at  all,  meaning  that  most

of  the  growth  in  data  examples  processed  can  be  used  for  increased  batch  sizes .

can  be  fit  very  well  with  a  power-law

0 . 7 3

N ( Cmin )   ∝  ( Cmin ) 
 .   (6 . 1 )

In  Figure   1 2,  we  show  the  effect  of  training  models  of  sub-optimal  sizes  (see  Appendix  B .4) .

By  definition  Cmin  ≡  6NBcrit S,  and  so  we  can  use  N ( Cmin )   to  extract  further  results .  In  particular,  since

− 4 . 8
 − 0 . 0 5
 0 . 2 4

prior  fits  show  B  ∝  L
 and  L  ∝  C
min  ,  we  can  conclude  that  Bcrit  ∝  C
min  .  This  leads  us  to  conclude

that  the  optimal  number  of  steps  will  only  grow  very  slowly  with  compute,  as

0 . 0 3

Smin  ∝  ( Cmin ) 
 ,  (6 . 2)

matching  the  empirical  results  in  Figure   1 4 .  In  fact  the  measured  exponent  is  sufficiently  small  that  our  results

may  even  be  consistent  with  an  exponent  of  zero .

Thus  we  conclude  that  as  we  scale  up  language  modeling  with  an  optimal  allocation  of  computation,  we

should  predominantly  increase  the  model  size  N,  while  simultaneously  scaling  up  the  batch  size  via  B  ∝

Bcrit  with  negligible  increase  in  the  number  of  serial  steps .  Since  compute-efficient  training  uses  relatively

few  optimization  steps,  additional  work  on  speeding  up  early  training  dynamics  may  be  warranted.

6.2  Predictions  from  L (N,  Smin )

The  results  for  L ( Cmin )   and  the  allocations  can  be  predicted  from  the  L (N,  Smin )   equation  obtained  in

Section  5 .  Given  our  equation  for  L (N,  Smin ) ,  we  can  substitute  Smin  =
 6CNmBin
  and  then  find  the  minimum

of  the  loss  as  a  function  of  N,  while  fixing  the  training  compute.  We  carry  out  this  procedure  in  detail  in

Appendix  B ,  where  we  also  provide  some  additional  predictions .

For  the  loss  as  a  function  of  training  compute,  we  predict  that

m i n
 α
mC
 i n

C
c

 Cmin  

L ( Cmin )  =  (6 . 3 )

where

m i n
 1

α
C  ≡
 ≈  0 . 0 5 4  (6 . 4)

1 /αS  +  1 /αB  +  1 /αN

in  excellent  agreement  with  the  exponent  of  Figure   1 3 .  We  also  predict  that

α
mC  i n
 / α N  ≈ 0 . 7 1

N ( Cmin )   ∝  ( Cmin ) 
   ( Cmin ) 
 (6 . 5 )

which  also  matches  the  scaling  of  Figure   1 4  to  within  a  few  percent.  Our  scaling  laws  provide  a  predictive

framework  for  the  performance  of  language  modeling.

1 6

Th e i ntersecti o n po i nt is sensitive to

the precise power- law parameters

Figure  15  Far  beyond  the  model  sizes  we  study  empirically,  we  find  a  contradiction  between  our  equations

for  L ( Cmin )  and  L (D )  due  to  the  slow  growth  of  data  needed  for  compute-efficient  training .  The  intersection

marks  the  point  before  which  we  expect  our  predictions  to  break  down.  The  location  of  this  point  is  highly

sensitive  to  the  precise  exponents  from  our  power-law  fits .

6.3  Contradictions  and  a  Conjecture

We  observe  no  signs  of  deviation  from  straight  power-law  trends  at  large  values  of  compute,  data,  or  model

size.  Our  trends  must  eventually  level  off,  though,  since  natural  language  has  non-zero  entropy.

Indeed,  the  trends  for  compute-efficient  training  described  in  this  section  already  contain  an  apparent  contra

diction.  At  scales  several  orders  of  magnitude  above  those  documented  here,  the  performance  predicted  by

the  L ( Cmin )   scaling  law  decreases  below  what  should  be  possible  given  the  slow  growth  in  training  data  with

compute.  This  implies  that  our  scaling  laws  must  break  down  before  this  point,  but  we  conj ecture  that  the

intersection  point  has  a  deeper  meaning :  it  provides  an  estimate  of  the  point  at  which  Transformer  language

models  reach  maximal  performance.

Since  the  amount  of  data  used  by  compute-efficient  training  grows  slowly  with  the  compute  budget,  the

performance  predicted  by  L ( Cmin )   eventually  hits  a  lower  bound  set  by  the  L (D )   power  law  (see  Figure   1 5) .

Let  us  work  this  out  in  more  detail .

To  keep  overfitting  under  control,  the  results  of  Section  4  imply  that  we  should  scale  the  dataset  size  as

0 . 74  0 . 5 4

D  ∝  N
 ∝  C
min  (6 . 6)

where  we  have  used  the  compute-efficient  N ( Cmin )  from  Figure   1 4.

Let  us  compare  this  to  the  data  requirements  of  compute-efficient  training .  If  we  train  at  the  critical  batch

size  (i.e.  C  =  2 Cmin )  and  never  re-use  data  during  training,  we  find  that  data  usage  grows  with  compute  as

2 Cm i n
 
 1 0  
 - 0 . 2 6

D ( Cmin )  =  ≈
  4  ×   1 0 tokens ( Cmin /PF Day) (6.7)

6N ( Cmin )

This  is  the  maximum  rate  at  which  the  dataset  size  can  productively  grow  with  compute,  since  it  means  that

we  are  only  training  for  a  single  epoch.  But  it  grows  the  dataset  much  more  slowly  than  in  Equation  (6 . 6) .

It  appears  to  imply  that  compute-efficient  training  will  eventually  run  into  a  problem  with  overfitting,  even  if

the  training  proces s  never  re-uses  any  data !

According  to  Figure   1 ,  we  expect  that  when  we  are  bottlenecked  by  the  dataset  size  (ie  by  overfitting) ,  the

loss  should  scale  as  L ( D )   ∝  D − 0 . 095 .  This  implies  that  the  loss  would  scale  with  compute  as  L ( D ( Cmin ) )   ∝

− 0 . 0 3
 -

C
min  once  we  are  data limited.  Once  again,  we  have  a  contradiction,  as  this  will  eventually  intersect  with

− 0 . 0 5 0

our  prediction  for  L ( Cmin )   from  Figure   1 3 ,  where  we  found  a  scaling  L ( Cmin )   ∝  C
min  .

The  intersection  point  of  L (D ( Cmin ) )   and  L ( Cmin )   occurs  at

C
 ∗  ∼  1 04  PF-D ays  N
 ∗  ∼  1 0 1 2  parameters ,  D ∗  ∼  1 0 1 2  tokens ,  L ∗  ∼  1 . 7  nats/token  (6 . 8)

though  the  numerical  values  are  highly  uncertain,  varying  by  an  order  or  magnitude  in  either  direction  de

pending  on  the  precise  values  of  the  exponents  from  the  power-law  fits .  The  most  obvious  interpretation  is

that  our  scaling  laws  break  down  at  or  before  we  reach  this  point,  which  is  still  many  orders  of  magnitude

away  in  both  compute  and  model  size.

1 7

One  might  also  conj ecture  that  this  intersection  point  has  a  deeper  meaning .  If  we  cannot  increase  the  model

size  beyond  N∗  without  qualitatively  different  data  requirements,  perhaps  this  means  that  once  we  reach

C
m∗
 in  and  N ∗
 ,  we  have  extracted  all  of  the  reliable  information  available  in  natural  language  data.  In  this

interpretation,  L
∗  would  provide  a  rough  estimate  for  the  entropy-per-token7  of  natural  language.  In  this

scenario ,  we  would  expect  the  los s  trend  to  level  off  at  or  before  L
∗
 .

We  can  guess  at  the  functional  form  of  L ( Cmin )   as  it  levels  off  by  considering  a  version  of  our  training

dataset  with  added  noise.  For  example,  we  could  append  a  random  string  of  tokens  to  each  context  shown

to  the  model  to  artificially  boost  the  loss  by  a  constant  additive  factor.  Then,  the  distance  from  the  noise

floor  L − Lnoise  would  be  a  more  meaningful  performance  metric,  with  even  a  small  decrease  in  this  distance

potentially  representing  a  significant  boost  in  qualitative  performance.  Since  the  artificial  noise  would  affect

all  of  our  trends  equally,  the  critical  point  of  6 . 8  would  not  change  (aside  from  the  absolute  value  of  L
∗
) ,  and

may  be  meaningful  even  if  it  occurs  after  the  leveling  off.

7  Related  Work

Power  laws  can  arise  from  a  wide  variety  of  sources  [THK 1 8] .  Power-law  scalings  with  model  and  dataset

size  in  density  estimation  [Was06]  and  in  random  forest  models  [Bia 1 2]  may  be  connected  with  our  results .

These  models  suggest  that  power-law  exponents  may  have  a  very  rough  interpretation  as  the  inverse  of  the

number  of  relevant  features  in  the  data.

Some  early  [BB0 1 ,  Goo0 1 ]  work  found  power-law  scalings  between  performance  and  dataset  size.  More

recent  work  [HNA+ 1 7 ,  HAD 1 9]  also  investigated  scaling  between  model  size  and  data  size ;  their  work  is

perhaps  the  closest  to  ours  in  the  literature8
 .  Note,  however,  that  [HNA+ 1 7]  found  super-linear  scaling  of

dataset  size  with  model  size,  whereas  we  find  a  sub-linear  scaling .  There  are  some  parallels  between  our

findings  on  optimal  allocation  of  compute  and  [Kom 1 9] ,  including  power-law  learning  curves .  EfficientNets

[TL 1 9]  also  appear  to  obey  an  approximate  power-law  relation  between  accuracy  and  model  size.  Very  recent

work  [RRB S 1 9b]  studies  scaling  with  both  dataset  size  and  model  size  for  a  variety  of  datasets ,  and  fits  an

ansatz  similar  to  ours .

EfficientNet  [TL 1 9]  advocates  scaling  depth  and  width  exponentially  (with  different  coefficients)  for  optimal

performance  of  image  models,  resulting  in  a  power-law  scaling  of  width  as  a  function  of  depth.  We  find  that

for  language  models  this  power  should  be  roughly  one  when  scaling  up  (as  width/depth  should  remain  fixed) .

But  more  importantly,  we  find  that  the  precise  architectural  hyperparameters  are  unimportant  compared  to  the

overall  scale  of  the  language  model.  In  [VWB 1 6]  it  was  argued  that  deep  models  can  function  as  ensembles

of  shallower  models,  which  could  potentially  explain  this  finding.  Earlier  work  [ZK 1 6]  has  compared  width

and  depth,  and  found  that  wide  ResNets  can  outperform  deep  ResNets  on  image  classification.  Some  studies

fix  computation  per  data  example,  which  tends  to  scale  in  proportion  to  the  number  of  model  parameters,

whereas  we  investigate  scaling  with  both  model  size  and  the  quantity  of  training  computation.

Various  works  [AS 1 7,  BHMM 1 8]  have  investigated  generalization  in  highly  overparameterized  models,  find

ing  a  “j amming  transition”  [GJS + 1 9]  when  the  model  size  reaches  the  dataset  size  (this  may  require  training

many  orders  of  magnitude  beyond  typical  practice,  and  in  particular  does  not  use  early  stopping) .  We  do

not  observe  such  a  transition,  and  find  that  the  necessary  training  data  scales  sublinearly  in  the  model  size.

Expansions  in  the  model  size,  particularly  at  large  width  [JGH 1 8 ,  LXS + 1 9] ,  may  provide  a  useful  framework

for  thinking  about  some  of  our  scaling  relations .  Our  results  on  optimization,  such  as  the  shape  of  learning

curves,  can  likely  be  explained  using  a  noisy  quadratic  model,  which  can  provide  quite  accurate  predictions

[ZLN+ 1 9]  in  realistic  settings .  Making  this  connection  quantitative  will  require  a  characterization  of  the

Hessian  spectrum  [Pap 1 8 ,  GKX 1 9,  GARD 1 8] .

8  Discussion

We  have  observed  consistent  scalings  of  language  model  log-likelihood  loss  with  non-embedding  parameter

count  N,  dataset  size  D ,  and  optimized  training  computation  Cmin ,  as  encapsulated  in  Equations  ( 1 . 5)  and

( 1 .6) .  Conversely,  we  find  very  weak  dependence  on  many  architectural  and  optimization  hyperparameters .

Since  scalings  with  N,  D ,  Cmin  are  power-laws ,  there  are  diminishing  returns  with  increasing  scale.

7 Defining  words  using  the  wc  utility,  the  WebText2  dataset  has  1 . 4  tokens  per  word  and  4 . 3  characters  per  token.

8 After  this  work  was  completed,  [RRB S 1 9a]  also  appeared,  which  makes  similar  predictions  for  the  dependence  of

los s  on  both  model  and  dataset  size.

1 8

We  were  able  to  precisely  model  the  dependence  of  the  loss  on  N  and  D ,  and  alternatively  on  N  and  S,  when

these  parameters  are  varied  simultaneously.  We  used  these  relations  to  derive  the  compute  scaling,  magnitude

of  overfitting,  early  stopping  step,  and  data  requirements  when  training  large  language  models .  So  our  scaling

relations  go  beyond  mere  observation  to  provide  a  predictive  framework.  One  might  interpret  these  relations

as  analogues  of  the  ideal  gas  law,  which  relates  the  macroscopic  properties  of  a  gas  in  a  universal  way,

independent  of  most  of  the  details  of  its  microscopic  consituents .

It  is  natural  to  conj ecture  that  the  scaling  relations  will  apply  to  other  generative  modeling  tasks  with  a

maximum  likelihood  loss ,  and  perhaps  in  other  settings  as  well.  To  this  purpose,  it  will  be  interesting  to

test  these  relations  on  other  domains ,  such  as  images ,  audio,  and  video  models ,  and  perhaps  also  for  random

network  distillation.  At  this  point  we  do  not  know  which  of  our  results  depend  on  the  structure  of  natural

language  data,  and  which  are  universal.  It  would  also  be  exciting  to  find  a  theoretical  framework  from

‘ ’ ‘ ’

which  the  scaling  relations  can  be  derived:  a   statistical  mechanics  underlying  the   thermodynamics  we

have  observed.  Such  a  theory  might  make  it  possible  to  derive  other  more  precise  predictions,  and  provide  a

systematic  understanding  of  the  limitations  of  the  scaling  laws .

In  the  domain  of  natural  language,  it  will  be  important  to  investigate  whether  continued  improvement  on  the

loss  translates  into  improvement  on  relevant  language  tasks .  Smooth  quantitative  change  can  mask  maj or

“ ”

qualitative  improvements :   more  is  different .  For  example,  the  smooth  aggregate  growth  of  the  economy

provides  no  indication  of  the  specific  technological  developments  that  underwrite  it.  Similarly,  the  smooth

improvements  in  language  model  loss  may  hide  seemingly  qualitative  changes  in  capability.

Our  results  strongly  suggest  that  larger  models  will  continue  to  perform  better,  and  will  also  be  much  more

sample  efficient  than  has  been  previously  appreciated.  Big  models  may  be  more  important  than  big  data.

In  this  context,  further  investigation  into  model  parallelism  is  warranted.  Deep  models  can  be  trained  using

pipelining  [HCC+ 1 8] ,  which  splits  parameters  depth-wise  between  devices,  but  eventually  requires  increased

batch  sizes  as  more  devices  are  used.  Wide  networks  on  the  other  hand  are  more  amenable  to  parallelization

[SCP+ 1 8] ,  since  large  layers  can  be  split  between  multiple  workers  with  less  serial  dependency.  Sparsity

[CGRS 1 9,  GRK 1 7]  or  branching  (e.g.  [KSH 1 2] )  may  allow  for  even  faster  training  of  large  networks  through

increased  model  parallelism.  And  using  methods  like  [WRH 1 7 ,  WYL 1 9] ,  which  grow  networks  as  they  train,

it  might  be  possible  to  remain  on  the  compute-efficient  frontier  for  an  entire  training  run.

Acknowledgements

We  would  like  to  thank  Shan  Carter,  Paul  Christiano,  Jack  Clark,  Aj eya  Cotra,  Ethan  Dyer,  Jason  Eisner,

Danny  Hernandez,  Jacob  Hilton,  Brice  Menard,  Chris  Olah,  and  Ilya  Sutskever  for  discussions  and  for  feed

back  on  drafts  of  this  work.

1 9

## A endices

# pp

### A  Summary  of  Power  Laws

#### For  easier  reference,  we  provide  a  summary  below  of  the  key  trends  described  throughout  the  paper.

###### Parameters  Data  Compute  Batch  Size  Equation

N  ∞  ∞  Fixed  L (N)  =  (Nc /N)
αN

∞  D  Early  Stop  Fixed  L (D )  =  (Dc /D )
α D

Optimal  ∞  C  Fixed  L ( C)  =   ( Cc / C) 
α C
 (naive)

m i n
 α
mC
 i n

Nopt  Dopt  Cmin  B    Bcrit  L ( Cmin )  =   C
c  / Cmin 

α N
 α D

N  D  Early  Stop  Fixed  L ( N,  D )  =    NNc
 
  α D  +
 DDc

  α 
  α S

N  ∞  S  steps  B  L ( N,  S)  =  Nc
 N  +
 Sc

  N
   Smi n ( S, B ) 
 

Table  4

#### The  empirical  fitted  values  for  these  trends  are :

#### Power  Law  Scale  (tokenization-dependent)

αN  =  0 . 076  Nc  =  8 . 8  ×   1 0 1 3  params  (non-embed)

αD  =  0 . 09 5  Dc  =  5 . 4  ×   1 0 1 3  tokens

αC  =  0 . 05 7  Cc  =  1 . 6  ×   1 07  PF-days

α
mC  in
 =  0 . 0 5 0  C
cm  in
 =  3 . 1   ×   1 08  PF-days

α B  =  0 . 2 1  B∗  =  2 . 1  ×   1 08
 tokens

α S  =  0 . 76  Sc  =  2 . 1   ×   1 03
 step s

Table  5

#### The  optimal  parameters  for  compute  efficient  training  are  given  by :

#### Compute-Efficient  Value  Power  Law  Scale

Nopt  =  Ne  ·  C
mpNin
   pN  =  0 . 73  Ne  =   1 . 3  ·  1 09  params

B    Bcrit  =
 1B/ α∗
 B  =  Be C
mp Bin
   pB  =  0 . 24  Be  =  2 . 0  ·   1 06
 tokens

L

Smin  =  Se  ·   C
mp Si
n  (lower  bound)  pS  =  0 . 03  Se  =  5 . 4  ·   1 03
 steps

Dopt  =  De  ·   C
mpDin
   ( 1  epoch)  pD  =  0 . 2 7  De  =  2  ·   1 0 1 0  tokens

Table  6

### B  Empirical  Model  of  Compute-Efficient  Frontier

#### Throughout  this  appendix  all  values  of  C,  S,  and  αC  are  adj usted  for  training  at  the  critical  batch  size  Bcrit .

###### ‘ ’

#### We  have  left  off  the   adj  label  to  avoid  cluttering  the  notation.

#### B.1  Defining  Equations

#### The  power-law  fit  to  the  learning  curves  implies  a  simple  prescription  for  compute-efficient  training .  In  this

#### appendix,  we  will  derive  the  optimal  performance,  model  size,  and  number  of  training  steps  as  a  function  of

###### 20

###### the  compute  budget.  We  start  with  the  Equation  ( 1 . 6) ,  repeated  here  for  convenience :

α N
 
 α S

Nc
 Sc

##  N
   S
 

###### L ( N,  S )  =  +
 .   (B . 1 )

###### Here,  S  represents  the  number  of  parameter  updates  when  training  at  the  critical  batch  size  [MKAT 1 8] ,

# which  was  defined  in  Equation  (5 . 2)9
 :

B∗

###### B  ( L )  =  1 α .   (B . 2)

L / B

###### We  would  like  to  determine  optimal  training  parameters  for  a  fixed  compute  budget,  so  we  replace  S  =

# C/  ( 6NB  (L) ) ,  where  C  is  the  number  of  FLOPs  used  in  the  training  run:

α N
 
 α S

Nc
 N

##  N
   L / B  C
 

###### L ( N,  C)  =  +
 6 B∗ Sc
 1 α .   (B . 3 )

# Now,  we  set  ∂N  L
  
C
 =  0  to  find  the  condition  for  optimality :

∂ L

### 0   =

### ∂N
  C

α N
 
 α S

− α N
 Nc
 α S
 N
 − N
 ∂L  ✚

### N
  N
  N
  L / B  C
   L ✚∂N
 
 

=  
 +
 6 B∗ Sc
 1 α 1     5
 ✚
 C

α N
 
 α S

αN
 Nc
 N

### αS
  N
   L / B  C
 

## =⇒
 =
 6B∗ Sc
 1 α (B .4)

###### Equation  (B . 3)  and  (B .4)  together  determine  the  compute-efficient  frontier.

# B.2  Efficient  Training

###### Now  we  as semble  the  implications  of  (B . 3 )  and  (B .4) .  First,  note  that  inserting  (B .4)  into  (B . 3 )  yields

αN

##  αS
 

###### L ( Neff  ( C) ,  C)  =  1  +
 L ( Neff ,  ∞ ) ,  (B . 5)

# which  implies  that  for  compute-efficient  training,  we  should  train  to  a  fixed  percentage  ααN
 ≈  1 0%  above

’ S

###### the  converged  loss .  Next,  let s  determine  how  the  optimal  loss  depends  on  the  compute  budget.  Eliminating

###### N  yields  a  power-law  dependence  of  performance  on  compute:

α C

Cc

##  C
 

###### L ( C)  =  (B . 6)

# where  we  defined

###### αC  =  1 /  ( 1 / αS  +  1 / αB  +  1 / αN  )   ≈  0 . 05 2  (B .7)

1 / α S + 1 / α N  
 1 / α S

α N
 α S

###### Cc  =  6Nc B∗ Sc
 1  +
 .   (B . 8 )

##  αS
   αN
 

# Similarly,  we  can  eliminate  L  to  find  N  ( C) :

## and

α C  / α N  
 1 / α N

### N  ( C) 
 C
 α N

## Nc
  Cc
   αS
 

## =
 1  +
 (B . 9 )

− 1 / α N  
 α C  / α S

Cc
 α N
 C

## 6NcB∗
  αS
   Cc
 

###### S  ( C )  =  1  +
 (B . 1 0)

# 9 There  is  a  slight  ambiguity  here : ˜ we  can  imagi˜ne  training  either  at  a  constant  batch  size  B  ( Lt arget ) ,  or  we  could

# instead  train  at  a  variable  batch  size  B  ( L ) ,  where  B  is  the  instantaneous  critical  batch  size  (as  opposed  to  B ,  which  is

# the  averaged  version) .  These  two  prescriptions  result  in  the  same  number  of  steps ,  so  we  can  ignore  this  subtlety  (see

###### [MKAT 1 8] ) .

###### 2 1

B.3  Comparison  to  Inefficient

Typically,  researchers  train  models  until  they  appear  to  be  close  to  convergence.  In  this  section,  we  compare

the  efficient  training  procedure  described  above  to  this  more  typical  setup .  We  define  a  the  convergence  factor

f  as  the  percent  deviation  from  the  converged  loss :

L ( N,  C)  =   ( 1  +  f ) L ( N,  ∞ ) .  (B . 1 1 )

For  compute-efficient  training  we  have  f  =  αN  /αS  ≈  1 0%  from  the  previous  section,  but  researchers

typically  use  a  much  smaller  value.  Here,  we  choose  f
 0  =  2 %  as  an  estimate.  For  a  fixed  value  of  the  los s ,

we  predict:

1 / α N

Nf
 1  +  f

Nf
 0
  1  +  f
 

=
 0
 ≈  2 . 7  (B . 1 2)

1 
 1 / α S

Sf
 1  +  f

f !

=
 1
 ≈  0 . 1 3  (B . 1 3 )

Sf
 0
 1   +   
 0

Cf
 = Nf
 Sf

≈  0 . 3 5  (B . 1 4)

Cf
 0
 Nf
 0
 Sf
 0

So  that  compute-efficient  training  uses  7 .7x  fewer  parameter  updates,  2.7x  more  parameters,  and  65 %  less

compute  to  reach  the  same  loss .

B.4  Suboptimal  Model  Sizes

We  can  solve  A. 1  to  find  an  expression  for  the  amount  of  compute  needed  to  reach  a  given  value  of  the  loss

L  with  a  model  of  size  N :

α N  − 1 / α S

N
 − Nc

 L / B
    N
  

C  ( N,  L )  =  6 B∗ Sc
 1 α L  
 .   (B . 1 5 )

Using  A. 6  and  A. 9,  we  can  eliminate  L  in  favor  of  Neff  (L) ,  the  model  size  which  reaches  L  most  efficiently.

From  there,  we  find  an  expression  for  the  excess  compute  needed  as  a  consequence  of  using  a  suboptimal

model  size :

α N  − 1 / α S

C  (N,  Neff ) 
 N
 αS
 − Neff

= 
 1   + 
 1   
 .   (B . 1 6 )

C  (Neff ,  Neff )
 Neff   αN
   N
   

The  result  is  shown  in  Figure  X.  Models  between  0. 6x  and  2. 2x  the  optimal  size  can  be  used  with  only  a

20%  increase  in  compute  budget.  Using  a  smaller  model  is  useful  when  accounting  for  the  cost  inference.  A

larger  model  can  be  trained  the  the  same  level  of  performance  in  fewer  steps,  allowing  for  more  parallelism

and  faster  training  if  sufficient  harware  is  available  (see  Figure  Y) :

α N  − 1 / α S

S  (N,  Neff ) 
 αS
 − Neff

S  (Neff ,  Neff )
  αN
   N
   

= 
 1   + 
 1   
 .   (B . 1 7 )

A  2. 2x  larger  model  requires  45 %  fewer  steps  at  a  cost  of  20%  more  training  compute.  Note  that  this  equation

should  not  be  trusted  for  very  large  models ,  as  it  is  only  valid  in  the  power-law  region  of  the  learning  curve

after  initial  transient  effects .

C  Caveats

In  this  section  we  list  some  potential  caveats  to  our  analysis .

•  At  present  we  do  not  have  a  solid  theoretical  understanding  for  any  of  our  proposed  scaling  laws .

The  scaling  relations  with  model  size  and  compute  are  especially  mysterious .  It  may  be  possible  to

understand  scaling  at  very  large  D  holding  model  size  fixed  [AS 1 7] ,  and  also  the  shape  of  learning

curves  late  in  training,  by  modeling  the  loss  with  a  noisy  quadratic .  But  the  scaling  with  D  at  very

large  model  size  still  remains  mysterious .  Without  a  theory  or  a  systematic  understanding  of  the

’

corrections  to  our  scaling  laws ,  it s  difficult  to  determine  in  what  circumstances  they  can  be  trusted.

22

6

Te st Lo s s 
 1 0 1 0
 )

# E arly Stopping Step 
 5 
 Train Loss 
 sn

# e

# k

# o

1 0 5 
 
 D ata S ize 
 ss 4
 
 1 0 9 
 
 (Te

2 1 M 
 o iz

p 
 43M 
 L S

tos 
 8 6M 
 t

S 4 
 1 7 2 M 
 3 
 es

1 0 
 344M 
 8 
 ta

68 8M 
 1 0 
 a

1 . 4B 
 D

1 0 3 
 
 2

1 0 3 
   1 0 4 
   1 0 5 
 
 1 0 3 
   1 0 4 
   1 0 5

Sc ×  [ L  ( N , D  )   L ( N ,   ) ]   1/   S 
 S te p

Figure  16  Left:  We  characterize  the  step  on  which  early  stopping  occurs ,  as  a  function  of  the  extent  of

overfitting .  The  red  line  indicates  a  lower  bound  for  early  stopping  that  is  derived  in  Section  5 . 3 .  Right:

We  display  train  and  test  loss  for  a  series  of  300M  parameter  models  trained  on  different  sized  dataset  sub

samples .  The  test  loss  typically  follows  that  of  a  run  done  with  unrestricted  data  until  diverging .  Note  that  the

degree  of  overfitting  (as  compared  to  the  infinite  data  limit)  is  significantly  overestimated  by  Ltest  −  Ltrain

(denoted  by  a  black  bar  for  each  run) .

•  We  are  not  especially  confident  in  the  prediction  of  Bcrit ( L )   for  values  of  the  loss  far  outside  the

range  we  have  explored.  Changes  in  Bcrit  could  have  a  significant  impact  on  trade-offs  between

data  parallelism  and  the  number  of  serial  training  steps  required,  which  would  have  a  maj or  impact

on  training  time .

•  We  did  not  thoroughly  investigate  the  small  data  regime,  and  our  fits  for  L ( N,  D )   were  poor  for

the  smallest  values  of  D  (where  an  epoch  corresponded  to  only  40  steps) .  Furthermore,  we  did

not  experiment  with  regularization  and  data  augmentation.  Improvements  in  these  could  alter  our

results ,  quantitatively  or  qualitatively.

•  We  used  the  estimated  training  compute  C  ≈  6NBS,  which  did  not  include  contributions  propor

tional  to  nctx  (see  Section  2 . 1 ) .  S o  our  scalings  with  compute  may  be  confounded  in  practice  in  the

regime  of  very  large  nctx ,  specifically  where  nctx  &  1 2dmodel .

•  We  tuned  learning  rates,  and  we  experimented  with  learning  rate  schedules .  But  we  may  have

neglected  to  tune  some  hyperparameter  (e.g.  intialization  scale  or  momentum)  that  have  an  important

effect  on  scaling .

•  The  optimal  choice  of  learning  rate  is  sensitive  to  the  target  loss .  When  training  close  to  convergence,

it  may  be  necessary  to  use  a  smaller  learning  rate  to  avoid  divergences .  But  when  conducting  a  short

training  run  (eg  due  to  compute  limitations) ,  it  may  be  possible  to  use  a  larger  learning  rate.  We  did

not  experiment  with  higher  learning  rates  for  training  runs  that  did  not  proceed  to  convergence.

## D  Supplemental  Figures

D. 1  Early  Stopping  and  Test  vs  Train

In  section  5 . 3  we  described  the  result  shown  in  Figure   1 6,  which  provides  a  prediction  for  a  lower  bound  on

the  early  stopping  step .  We  also  show  the  train  and  test  loss  for  a  given  model  size  when  training  on  different

sized  datasets .

D.2  Universal  Transformers

We  compare  the  performance  of  standard  Transformers  to  recurrent  Transformers  [DGV+ 1 8]  in  Figure   1 7 .

These  models  re-use  parameters ,  and  so  perform  slightly  better  as  a  function  of  N,  but  slightly  worse  as  a

function  of  compute  C.  We  include  several  different  different  possibilities  for  parameter  re-use.

D.3  Batch  Size

We  measure  the  critical  batch  size  using  the  data  displayed  in  figure   1 8 .  This  made  it  possible  to  estimate

Bcrit ( L )   in  figure   1 0 .

23

4 . 5 
 4 . 5 
 2 x Reus e

### 4x Reuse

4 . 0 
 4 . 0 
 8x Reus e

### ss 
 ss 
 Non-recurrent Models

Lo 3 . 5 
 Lo 3 . 5

###### t  t

###### s s

Te 3 . 0 
 2x Reuse 
 Te 3 . 0

### 4x Reuse

### 8x Reuse

### 2 . 5 
 Non-recurrent Models 
 2 . 5

1 0 5 
   1 0 6 
   1 0 7 
   1 0 8 
   1 0 9 
 
 1 0 5 
   1 0 6 
   1 0 7 
   1 0 8 
   1 0 9

## Parameters, including reuse (non-embedding) 
 Parameters (non-embedding)

Figure  17  We  compare  recurrent  Transformers  [DGV+ 1 8] ,  which  re-use  parameters,  to  standard  Trans

formers .  Recurrent  Transformers  perform  slightly  better  when  comparing  models  with  equal  parameter  count,

but  slightly  worse  when  accounting  for  reuse  and  comparing  per  FLOP.

# Batch Size Scan - 3M Params 
 Batch Size Scan - 8 5M Params

1 0 1 1
 
 1 0 
 1 0

d 1 0
 1 0
 
 d 1 0
 1 0

es 8 
 es 8

se 1 0 9 
 
 ss 
 se ss

###### co o co o

Pr 8 
 Lt  Pr 1 0 8 
 
 Lt

s  1 0 
 6 
 s s  6 
 s

###### n Te n Te

ke 1 0 7 
 
 ke

To 4 
 To 1 0 6 
 
 4

1 0 6

1 0 2 
   1 0 3 
   1 0 4 
   1 0 5 
 
 1 0 1 
   1 0 2 
   1 0 3 
   1 0 4 
   1 0 5

S tep 
 S tep

Figure  18  These  figures  demonstrate  fits  to  Equation  (5 . 1 )  for  a  large  number  of  values  of  the  loss  L,  and

for  two  different  Transformer  model  sizes .  These  fits  were  used  to  measure  Bcrit (L)   for  Figure   1 0.

D.4  Sample  Efficiency  vs  Model  Size

It  is  easy  to  see  from  figure  2  that  larger  models  train  faster,  and  are  therefore  more  sample  efficient.  We

provide  another  way  of  looking  at  this  phenomenon  in  figure   1 9,  which  shows  when  different  models  reach

various  fixed  values  of  the  los s .

5 . 5 
 1 1 
 5 . 5

5 
 )in 1
 0

)in 1
 0 
 5 . 0 
 m 5 . 0

m (E

(S 
 4 . 5 
 lse 1  0
 1 0
 
 4 . 5

sp   
 p

Ste 1
 0
 4 
 
 4 . 0 ss
 
 ma 
 
 4 . 0 ss

m 
 
 Lo Ex 1
 
0 9 
 
 Lo

u 
 3 . 5 
 m 
 3 . 5

im 
 u

in 3 . 0 
 im 
 3 . 0

M 1 0 3 
 
 in 1 0 8

2 . 5 
 M 2 . 5

1 0 6 
   1 0 7 
   1 0 8 
 
 1 0 6 
   1 0 7 
   1 0 8

## Parameters (non-embedding) 
 Parameters (non-embedding)

Figure  19  The  number  of  minimum  serial  steps  needed  to  reach  any  fixed  value  of  the  test  loss  decreases

precipitously  with  model  size.  S ample  efficiency  (show  here  for  training  far  below  the  critical  batch  size)

improves  greatly  as  well,  improving  by  a  factor  of  almost   1 00  when  comparing  the  smallest  possible  model

to  a  very  large  one .

24

# Per-token Loss ( 7 74M Params) 
 3

1 0

8 
 4 . 0 +  3 .  2 T  0
 . 4 7 
 1 0

s 
 3 . 4 +  4 .  0 T  0
 . 5 6

so 7 
 2 . 9 +  4 .  5 T  0
 . 5 6 
 8 
sr 
 x

L  2 . 7 +  4 .  9 T  0
 . 60 
 1 0 
 te s 
 8 
 1 0 2 
 
 e

ts 6 
 2 . 4 +  5 .  1 T  0
 . 6 1 
 e so dn

Te 2 . 3 +  5 .  4 T  0
 . 62 
 mar L  I

ne  5 
 7 P
a tse 6 
 ne

k 1 0 
l  T 1 k
 o

To 4 
 de 1 0 
 T

re- o 4

# P M

3 
 1 0 6

2 
 1 0 0

1 0 0 
   1 0 1 
   1 0 2 
   1 0 3 
 
 1 0 1 
   1 0 3 
   1 0 5

### Token Index 
 Step

Figure  20  This  figure  provides  information  about  the  performance  per  token  as  a  function  of  model  size

and  training  time.  Left:  Los s  per  token  as  a  function  of  its  position  T  in  the   1 024-token  context.  Los s  scales

predictably  as  a  power-law  in  T.  Right:  Test  los s  per  token  as  a  function  of  training  step .

7 . 5 
 Token  1 / 1 0 2 4

Token 2/1 0 2 4

6 . 0 
 Token 4/ 1 0 2 4

s 
 Token 8/ 1 0 2 4

so Token 1 6/1 024

L  4 . 5 
 Token 64/1 0 2 4

ts Token 2 5 6/1 024

e Token 1 0 24/1 0 24

T Token 1 /8

Token 2/8

3 . 0 
 Token 4/8

Token 8/8

1 0 4 
   1 0 5 
   1 0 6 
   1 0 7 
   1 0 8 
   1 0 9

## Parameters (excl. embedding)

Figure  21  In  addition  to  the  averaged  loss,  individual  tokens  within  the   1 024-token  context  also  improve

smoothly  as  model  size  increases .  Training  runs  with  shorter  context  nctx  =  8  (dashed  lines)  perform  better

on  early  tokens ,  since  they  can  allocate  all  of  their  capacity  to  them.

D.5  Context  Dependence

The  trends  for  los s  as  a  function  of  model  size  are  displayed  for  different  tokens  in  the  context  in  Figure  2 1 .

We  see  that  models  trained  on  nctx  =  1 024  show  steady  improvement  with  model  size  on  all  but  the  first

token .

Fixing  model  size,  it  appears  that  the  los s  scales  as  a  power-law  as  a  function  of  position  T  in  the  context,  see

Figure  20.  This  may  be  a  consequence  of  underlying  power-law  correlations  in  language  [EP94,  ACDE 1 2,

LT 1 6] ,  or  a  more  general  feature  of  the  model  architecture  and  optimization.  It  provides  some  suggestion  for

the  potential  benefits  (or  lack  thereof)  from  training  on  larger  contexts .  Not  only  do  larger  models  converge

to  better  performance  at  T  =  1 024,  but  they  also  improve  more  quickly  at  early  tokens ,  suggesting  that  larger

models  are  more  efficient  at  detecting  patterns  with  less  contextual  information.  In  the  right-hand  plot  we

show  how  per-token  performance  varies  for  a  fixed  model  as  a  function  of  the  training  step .  The  model  begins

by  learning  short-range  information,  and  only  learns  longer-range  correlations  later  in  training .

We  have  also  included  models  trained  with  a  tiny  context  nctx  =  8  in  order  to  compare  with  our  longer

context  models .  Even  modestly  sized  models  trained  on  nctx  =  8  can  dominate  our  largest  nctx  =  1 024

models  on  very  early  tokens .  This  also  suggests  that  further  improvements  should  be  possible  with  much

larger  models  trained  on  large  contexts .

D.6  Learning  Rate  Schedules  and  Error  Analysis

We  experimented  with  a  variety  of  learning  rates  and  schedules .  A  host  of  schedules  and  resulting  test

performances  for  a  small  language  model  are  plotted  in  Figure  22.  We  conclude  that  the  choice  of  learning

rate  schedule  is  mostly  irrelevant,  as  long  as  the  total  summed  learning  rate  is  sufficiently  large,  and  the

schedule  includes  a  warmup  period  and  a  final  decay  to  near-vanishing  learning  rate.  Variations  among

25

0 . 0 0 1 0 
 3 . 9 0

0 . 0 0 0 8 
 3 . 8 5

te

# a

Rg  0 . 0 0 0 6 
 s 3 .
 8 0

# in so

### nra 0 . 0 0 04 
 L 3 . 7 5

# Le

0 . 0 0 0 2 
 3 . 7 0

0 . 0 0 0 0 
 3 . 6 5

### 0   5 0 0 0 0   1 0 0 0 0 0   1 5 0 0 0 0   2 0 0 0 0 0   2 5 0 0 0 0 
 5 0   1 0 0   1 5 0   2 0 0   2 5 0

## Step 
 LR Summed Over Steps

Figure  22  We  test  a  variety  of  learning  rate  schedules  including  cosine  decay,  linear  decay,  as  well  as  other

faster/slower  decays  schedules  on  a  3  million  parameter  model,  shown  on  the  left.  For  these  experiments  we

do  not  decay  to  zero,  since  we  find  that  this  tends  to  give  a  fixed  improvement  close  to  the  end  of  training .

We  find  that,  as  long  as  the  learning  rate  is  not  too  small  and  does  not  decay  too  quickly,  performance  does

not  depend  strongly  on  learning  rate.  Run-to-run  variation  is  at  the  level  of  0 . 05  in  the  los s ,  so  averaging

multiple  runs  is  necessary  to  validate  performance  changes  smaller  than  this  level.

)e 6
 
 L =  ( N  /8 . 8   1 0 1 3
 ) 
   0 . 0 7 6

# c

ne L =    0 . 2 5 l o g ( N/7 . 1   1 0 1 2
 )

gr 5

# e

# v

# n

# oc

t  4

# (a

# s

# s

# o

L  3

# ts

# e

# T

2 
 1 0 4 
   1 0 5 
   1 0 6 
   1 0 7 
   1 0 8 
   1 0 9

# Parameters (non-embedding)

Figure  23  The  trend  for  performance  as  a  function  of  parameter  count,  L (N) ,  is  fit  better  by  a  power  law

than  by  other  functions  such  as  a  logarithm  at  a  qualitative  level.

schedules  appear  to  be  statistical  noise,  and  provide  a  rough  gauge  for  the  scale  of  variation  between  different

training  runs .  Experiments  on  larger  models  suggest  that  the  variation  in  the  final  test  loss  between  different

random  seeds  is  roughly  constant  in  magnitude  for  different  model  sizes .

We  found  that  larger  models  require  a  smaller  learning  rate  to  prevent  divergence,  while  smaller  models  can

tolerate  a  larger  learning  rate.  To  implement  this ,  the  following  rule  of  thumb  was  used  for  most  runs :

LR (N)  ≈  0 . 003239  +  − 0 . 000 1 395  log (N)  (D . 1 )

We  expect  that  this  formula  could  be  improved.  There  may  be  a  dependence  on  network  width,  likely  set  by

the  initialization  scale.  The  formula  also  breaks  down  for  N  >  1 0 1 0  parameters .  Nevertheless,  we  found  that

it  works  sufficiently  well  for  the  models  we  considered.

D.7  Fit  Details  and  Power  Law  Quality

We  experimented  with  a  number  of  functional  forms  for  the  fits  to  L (N) ,  L ( C) ,  and  L (D ) ;  the  power-law

fits  were  qualitatively  much  more  accurate  than  other  functions  such  as  logarithms  (see  Figure  23) .

For  L ( C) ,  we  do  not  include  small  models  with  only   1  layer  in  the  fit,  as  the  transition  from   1  to  2  layers

causes  a  noticable  lump  in  the  data.  For  L ( N)   we  also  do  not  include  very  small  models  with  only   1  layer  in

the  fit,  and  we  exclude  the  largest  models  that  have  not  trained  fully  to  convergence.  Fit  parameters  change

marginally  if  we  do  include  them,  and  the  trend  extrapolates  well  in  both  directions  regardless .

D.8  Generalization  and  Architecture

In  figure  24  we  show  that  generalization  to  other  data  distributions  does  not  depend  on  network  depth  when  we

hold  the  total  parameter  count  fixed.  It  seems  to  depend  only  on  the  performance  on  the  training  distribution.

26

2 . 8

2 . 7 
 Wikipedia

ss 
 Books

Lo 2 . 6 
 Internet Books

ts  Common Crawl

Te 2 . 5 
 WebText2 (Train)

WebText2 (Test)

2 . 4

2 . 3

1 0 1 
   1 0 2

Depth

Figure  24  We  show  evaluations  on  a  series  of  datasets  for  models  with  approximately   1 . 5  Billion  param

eters .  We  observe  no  effect  of  depth  on  generalization;  generalization  performance  depends  primarily  on

training  distribution  performance.  The   1 2-layer  model  overfit  the  Internet  B ooks  dataset  and  we  show  the

early- stopped  performance ;  we  have  not  seen  this  surprising  result  in  other  experiments .

###### List  of  Figures

1  S ummary  o f   s imp l e  p ow er  l aw s .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3

2  Illu stration  of  s ample  efficiency  and  compute  efficiency.   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   4

3  How  to  s c ale  up  model  size ,  b atch  size ,  and  s eri al  step s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   4

4  Performance  when  varying  model  and  data  size,  or  model  and  training  steps,  simultaneously  5

5  Weak  dependence  of  performance  on  hyperparameter  tuning  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  8

6  Comparison  of  performance  trend  when  including  or  excluding  embeddings  .  .  .  .  .  .  .  .  .  8

7  LS TM  and  Tran sformer  performance  comp ari s on   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   9

8  Generali z ati on  to  other  te s t  data s et s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 0

9  Univer s ality  o f  overfi ttin g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 1

1 0  C ri ti c al  b atc h   s i z e   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 2

1 1  Performance  versus  compute  budget  or  number  of  parameter  updates  .  .  .  .  .  .  .  .  .  .  .  .  .  1 4

1 2  Training  on   s ub optimal  mo del s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 5

1 3  Compari son  between  empirical  and  adj usted  compute  trends  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  1 5

1 4  Optimal  model  size  and  serial  number  of  step s  versus  compute  budget  .  .  .  .  .  .  .  .  .  .  .  .  1 6

1 5  C ontradiction  between  c ompute  and  data  trends   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 7

1 6  Early  stopping  lower  bound  and  training  curves  for  overfit  models  .  .  .  .  .  .  .  .  .  .  .  .  .  .  23

1 7  Univer s al  tran s former s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   24

1 8  B atc h   s i z e   s c an s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   2 4

1 9  Another  l o ok  at   s ample  effi ci ency   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   24

20  Power-law  dependence  of  performance  on  po sition  in  context  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  25

2 1  Performance  at  different  context  po sitions  versu s  model  size  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  25

2 2  Le arning  rate   s che dul e   s c an   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   2 6

2 3  Comp ari s on  of  Power-Law  and  Logarithmic  Fits   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   26

24  Generali z ati o n  ver s u s  depth   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   2 7

27

# List  of  Tables

1  Parameter  and  c ompute  c ounts  for  Tran s former   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   7

2  Fi t s  to   L ( N ,   D )   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 1

3  Fi t s  to   L ( N ,   S )   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 4

4  Key  tren d  e qu ati o n s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   2 0

5  Key  p arameter s  to  trend  fi t s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   2 0

6  Trend s  for  c ompute - effi cient  training   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   20

References

[ACDE 1 2]  Eduardo  G  Altmann,  Giampaolo  Cristadoro,  and  Mirko  Degli  Esposti.  On  the  origin  of  long

range  correlations  in  texts .  Proceedings  of  the N  ational A  cademy  of  Sciences,   1 09(29) : 1 1 5 82–

1 1 5 87 ,  20 1 2 .  25

[AS 1 7]  Madhu  S .  Advani  and  Andrew  M.  Saxe.  High-dimensional  dynamics  of  generalization  error  in

neural  networks .  arXiv,  20 1 7 ,   1 7 1 0 . 03 667 .   1 1 ,   1 8 ,  22

[BB0 1 ]  Michele  B anko  and  Eric  Brill.  Scaling  to  very  very  large  corpora  for  natural  language  disam

biguation.  In  Proceedings  of the  39th  annual  meeting  on  association f  or  computational  linguis

tics,  pages  26–3 3 .  Association  for  Computational  Linguistics ,  200 1 .   1 8

[BHMM 1 8]  Mikhail  Belkin,  Daniel  Hsu,  Siyuan  Ma,  and  Soumik  Mandal.  Reconciling  modern  machine

learning  and  the  bias-variance  trade-off.  arXiv,  20 1 8 ,   1 8 1 2 . 1 1 1 1 8 .   1 8

[Bia 1 2]  GÃŠrard  Biau.  Analysis  of  a  random  forests  model.  Journal  of M  achine L  earning R  esearch,

1 3 (Apr) : 1 063– 1 095 ,  20 1 2 .   1 8

[CGRS 1 9]  Rewon  Child,  Scott  Gray,  Alec  Radford,  and  Ilya  Sutskever.  Generating  long  sequences  with

sparse  transformers .  CoRR,  abs/1 904. 1 0509,  20 1 9,   1 904. 1 0509 .  URL  http : / / arx iv . org/

ab s / 1 9 04 . 1 0 5 0 9 .   1 9

[DCLT 1 8]  Jacob  Devlin,  Ming-Wei  Chang,  Kenton  Lee,  and  Kristina  Toutanova.  Bert:  Pre-training  of  deep

bidirectional  transformers  for  language  understanding,  20 1 8 ,  arXiv : 1 8 1 0.04805 .  2

[DGV+ 1 8]  Mostafa  Dehghani,  Stephan  Gouws,  Oriol  Vinyals,  Jakob  Uszkoreit,  and  Lukasz  Kaiser.  Uni

versal  transformers .  CoRR,  abs/1 807 .03 8 1 9,  20 1 8 ,   1 807 .03 8 1 9 .  URL  http : / / arx iv . org/

ab s / 1 8 0 7 . 0 3 8 1 9 .  6 ,  9 ,  23 ,  24

[EP94]  Werner  Ebeling  and  Thorsten  Pöschel.  Entropy  and  long-range  correlations  in  literary  english.

EPL  (Europhysics L  etters) ,  26(4) : 24 1 ,   1 994 .  25

[Fou]  The  Common  Crawl  Foundation.  Common  crawl.  URL  http : / / c ommoncrawl . org.  7

[GARD 1 8]  Guy  Gur-Ari,  Daniel  A.  Roberts,  and  Ethan  Dyer.  Gradient  descent  happens  in  a  tiny  subspace.

20 1 8 ,  arXiv : 1 8 1 2 . 04754 .   1 8

[GJS + 1 9]  Mario  Geiger,  Arthur  Jacot,  Stefano  Spigler,  Franck  Gabriel,  Levent  S agun,  Stéphane  d’ Ascoli,

Giulio  Biroli,  Clément  Hongler,  and  Matthieu  Wyart.  Scaling  description  of  generalization  with

number  of  parameters  in  deep  learning .  arXiv,  20 1 9 ,   1 90 1 .0 1 608 .   1 8

[GKX 1 9]  Behrooz  Ghorbani,  Shankar  Krishnan,  and  Ying  Xiao.  An  investigation  into  neural  net  op

timization  via  hessian  eigenvalue  density.  CoRR,  abs/ 1 90 1 . 1 0 1 59,  20 1 9,   1 90 1 . 1 0 1 59 .  URL

http : / / arx iv . org/ ab s / 1 90 1 . 1 0 1 59 .   1 8

[Goo0 1 ]  Joshua  Goodman.  A  bit  of  progress  in  language  modeling.  CoRR,  cs .CL/0 1 08005 ,  200 1 .  URL

http : / / arxiv . org/ abs / c s . CL/0 1 08005 .   1 8

[GRK 1 7]  Scott  Gray,  Alec  Radford,  and  Diederik  P  Kingma.  Gpu  kernels  for  block-sparse  weights .  ope

nai. com,  20 1 7 .   1 9

[HAD 1 9]  Joel  Hestness,  Newsha  Ardalani,  and  Gregory  Diamos.  Beyond  human-level  accuracy:  Compu

tational  challenges  in  deep  learning.  In  Proceedings  of  the  24th  Symposium  on  Principles  and

Practice  of  Parallel  Programming,  PPoPP  ’ 1 9,  pages   1 – 1 4,  New  York,  NY,  USA,  20 1 9 .  ACM.

doi : 1 0. 1 1 45/3 293 8 8 3 . 3 2957 1 0.   1 8

28

[HCC+ 1 8]  Yanping  Huang,  Yonglong  Cheng,  Dehao  Chen,  HyoukJoong  Lee,  Jiquan  Ngiam,  Quoc  V.  Le,

and  Zhifeng  Chen.  Gpipe :  Efficient  training  of  giant  neural  networks  using  pipeline  parallelism.

CoRR,  abs/1 8 1 1 .06965 ,  20 1 8 ,   1 8 1 1 .06965 .  URL  http : / / arx iv . org/ ab s / 1 8 1 1 . 069 65 .   1 9

[HNA+ 1 7]  Joel  Hestness,  Sharan  Narang,  Newsha  Ardalani,  Gregory  Diamos,  Heewoo  Jun,  Hassan  Kia

ninej ad,  Md.  Mostofa  Ali  Patwary,  Yang  Yang,  and  Yanqi  Zhou.  Deep  learning  scaling  is  pre

dictable,  empirically,  20 1 7 ,   1 7 1 2 . 00409 .   1 8

[JGH 1 8]  Arthur  Jacot,  Franck  Gabriel,  and  Clément  Hongler.  Neural  tangent  kernel:  Convergence  and

generalization  in  neural  networks .  In  Advances  in  neural  information p  rocessing  systems,  pages

857 1 –85 80,  20 1 8 .   1 8

[KB 1 4]  Diederik  P.  Kingma  and  Jimmy  B a.  Adam:  A  method  for  stochastic  optimization,  20 1 4,

1 4 1 2 . 69 80 .  7

[Kom 1 9]  Aran  Komatsuzaki.  One  epoch  is  all  you  need,  20 1 9,  arXiv : 1 906 .06669 .   1 8

[KSH 1 2]  Alex  Krizhevsky,  Ilya  Sutskever,  and  Geoffrey  E.  Hinton.  Imagenet  classification  with  deep

convolutional  neural  networks .  In  Proceedings  of  the  25th I  nternational  Conference  on N  eural

Information  Processing  Systems  -  Volume  1 ,  NIPS ’ 1 2,  pages   1 097– 1 1 05 ,  USA,  20 1 2.  Curran

Associates  Inc.  URL  http : / /dl . acm . org/ c it at i on . cfm? id=2999 1 34 . 2999257 .   1 9

[LCG+ 1 9]  Zhenzhong  Lan,  Mingda  Chen,  Sebastian  Goodman,  Kevin  Gimpel,  Piyush  Sharma,  and  Radu

S oricut.  Albert:  A  lite  bert  for  self- supervised  learning  of  language  representations ,  20 1 9 ,

1 909 . 1 1 942 .  9

[LOG+ 1 9]  Yinhan  Liu,  Myle  Ott,  Naman  Goyal,  Jingfei  Du,  Mandar  Joshi,  Danqi  Chen,  Omer  Levy,  Mike

Lewis,  Luke  Zettlemoyer,  and  Veselin  Stoyanov.  Roberta:  A  robustly  optimized  BERT  pretrain

ing  approach.  CoRR,  abs/1 907 . 1 1 692,  20 1 9,   1 907 . 1 1 692.  URL  http : / / arx iv . org/ ab s /

1 9 0 7 . 1 1 6 9 2 .  2

[LSP+ 1 8]  Peter  J.  Liu,  Mohammad  Saleh,  Etienne  Pot,  Ben  Goodrich,  Ryan  Sepassi,  Lukasz  Kaiser,  and

Noam  Shazeer.  Generating  wikipedia  by  summarizing  long  sequences .  arXiv: 1 801 . 1 01 98  [cs],

20 1 8 ,   1 80 1 . 1 0 1 98 .  URL  http : / / arx iv . org/ ab s / 1 80 1 . 1 0 1 98 .  2,  6

[LT 1 6]  Henry  W  Lin  and  Max  Tegmark.  Criticality  in  formal  languages  and  statistical  physics .  arXiv

preprint  arXiv: 1 606. 0673 7,  20 1 6 .  25

[LXS + 1 9]  Jaehoon  Lee,  Lechao  Xiao,  Samuel  S .  Schoenholz,  Yasaman  B ahri,  Roman  Novak,  Jascha  Sohl

Dickstein,  and  Jeffrey  Pennington.  Wide  neural  networks  of  any  depth  evolve  as  linear  models

under  gradient  descent,  20 1 9 ,  arXiv : 1 902.06720.   1 8

[MKAT 1 8]  Sam  McCandlish,  Jared  Kaplan,  Dario  Amodei,  and  OpenAI  Dota  Team.  An  empirical  model

of  large-batch  training ,  20 1 8 ,  arXiv : 1 8 1 2 . 06 1 62 .  3 ,  5 ,  6 ,   1 2,   1 3 ,  2 1

[Pap 1 8]  Vardan  Papyan.  The  full  spectrum  of  deep  net  hessians  at  scale :  Dynamics  with  sample  size.

CoRR,  abs/1 8 1 1 .07062,  20 1 8 ,   1 8 1 1 .07062.  URL  http : / / arx iv . org/ ab s / 1 8 1 1 . 07062 .   1 8

[RNS S 1 8]  Alec  Radford,  Karthik  Narasimhan,  Tim  Salimans,  and  Ilya  Sutskever.  Improving  language

understanding  by  generative  pre-training.  URL  https://s3-us-west-2.  amazonaws.  com/openai

assets/research-covers/languageunsupervised/language  understanding p  aper. p  df,  20 1 8 .  2,  6

[RRB S 1 9a]  Jonathan  S .  Rosenfeld,  Amir  Rosenfeld,  Yonatan  Belinkov,  and  Nir  Shavit.  A  constructive

prediction  of  the  generalization  error  acros s  scales ,  20 1 9 ,   1 909 . 1 267 3 .   1 8

[RRB S 1 9b]  Jonathan  S .  Rosenfeld,  Amir  Rosenfeld,  Yonatan  Belinkov,  and  Nir  Shavit.  A  constructive

prediction  of  the  generalization  error  across  scales ,  20 1 9 ,  arXiv : 1 909 . 1 267 3 .   1 8

[RSR+ 1 9]  Colin  Raffel,  Noam  Shazeer,  Adam  Roberts,  Katherine  Lee,  Sharan  Narang,  Michael  Matena,

Yanqi  Zhou,  Wei  Li,  and  Peter  J.  Liu.  Exploring  the  limits  of  transfer  learning  with  a  unified

text-to-text  transformer,  20 1 9 ,  arXiv : 1 9 1 0 . 1 068 3 .  2

[RWC+ 1 9]  Alec  Radford,  Jeff  Wu,  Rewon  Child,  David  Luan,  Dario  Amodei,  and  Ilya  Sutskever.  Language

models  are  unsupervised  multitask  learners .  openai. com,  20 1 9 .  2,  5 ,  6,  7 ,  8

[SCP+ 1 8]  Noam  Shazeer,  Youlong  Cheng,  Niki  Parmar,  Dustin  Tran,  Ashish  Vaswani,  Penporn  Koanan

takool,  Peter  Hawkins,  HyoukJoong  Lee,  Mingsheng  Hong,  Cliff  Young,  Ryan  Sepassi,  and

Blake  Hechtman.  Mesh-tensorflow :  Deep  learning  for  supercomputers,  20 1 8 ,   1 8 1 1 .02084.   1 9

[SHB 1 5]  Rico  Sennrich,  B arry  Haddow,  and  Alexandra  Birch.  Neural  machine  translation  of  rare  words

with  subword  units .  CoRR,  20 1 5 ,   1 508 .07909 .  6

29

[SLA+ 1 8]  Christopher  J.  Shallue,  Jaehoon  Lee,  Joe  Antognini,  Jascha  Sohl-Dickstein,  Roy  Frostig,  and

George  E.  Dahl.  Measuring  the  effects  of  data  parallelism  on  neural  network  training,  20 1 8 ,

arXiv : 1 8 1 1 . 03 600 .   1 2

[S S 1 8]  Noam  Shazeer  and  Mitchell  Stern.  Adafactor:  Adaptive  learning  rates  with  sublinear  memory

cost.  CoRR,  abs/1 804.04235 ,  20 1 8 ,   1 804.04235 .  URL  http : / / arx iv . org/ ab s / 1 804 . 04235 .

7

[THK 1 8]  Stefan  Thurner,  Rudolf  Hanel,  and  Peter  Klimek.  Introduction  to  the  theory  of complex  systems.

Oxford  University  Press ,  20 1 8 .   1 8

[TL 1 9]  Mingxing  Tan  and  Quoc  V.  Le.  Efficientnet:  Rethinking  model  scaling  for  convolutional  neural

networks .  CoRR,  abs/1 905 . 1 1 946,  20 1 9,   1 905 . 1 1 946.  URL  http : / / arx iv . org/ ab s / 1 905 .

1 1 9 4 6 .   1 8

[VSP+ 1 7]  Ashish  Vaswani,  Noam  Shazeer,  Niki  Parmar,  Jakob  Uszkoreit,  Llion  Jones,  Aidan  N  Gomez,

Ł  ukasz  Kaiser,  and  Illia  Polosukhin.  Attention  is  all  you  need.  In  I.  Guyon,  U.  V.  Luxburg,

S .  B engio,  H.  Wallach,  R.  Fergus,  S .  Vishwanathan,  and  R.  Garnett,  editors,  Advances  in N  eural

Information  Processing  Systems  30,  pages  5998–6008 .  Curran  Associates,  Inc. ,  20 1 7 .  URL

http : / /papers . nips . c c /paper/7 1 8 1 - att ent i on- i s - al l - you- ne ed . pdf .  2,  6

[VWB 1 6]  Andreas  Veit,  Michael  Wilber,  and  Serge  Belongie.  Residual  networks  behave  like  ensembles

of  relatively  shallow  networks ,  20 1 6,  arXiv : 1 605 . 0643 1 .  8 ,   1 8

[Was06]  Larry  Wasserman.  All  of  nonparametric  statistics.  Springer  Science  &  Business  Media,  2006.

1 8

[WPN+ 1 9]  Alex  Wang,  Yada  Pruksachatkun,  Nikita  Nangia,  Amanpreet  Singh,  Julian  Michael,  Felix  Hill,

Omer  Levy,  and  Samuel  R.  Bowman.  Superglue:  A  stickier  benchmark  for  general-purpose

language  understanding  systems,  20 1 9,   1 905 .005 37 .  2

[WRH 1 7]  Yu-Xiong  Wang,  Deva  Ramanan,  and  Martial  Hebert.  Growing  a  brain:  Fine-tuning  by  in

creasing  model  capacity.  201 7 I  EEE  Conference  on  Computer  Vision  and  Pattern R  ecognition

(CVPR) ,  Jul  20 1 7 .  doi : 1 0 . 1 1 09/cvpr. 20 1 7 . 3 23 .   1 9

[WYL 1 9]  Wei  Wen,  Feng  Yan,  and  Hai  Li.  Autogrow :  Automatic  layer  growing  in  deep  convolutional

networks ,  20 1 9 ,   1 906 .02909 .   1 9

[YDY+ 1 9]  Zhilin  Yang,  Zihang  Dai,  Yiming  Yang,  Jaime  Carbonell,  Ruslan  Salakhutdinov,  and  Quoc  V.

Le.  Xlnet:  Generalized  autoregressive  pretraining  for  language  understanding,  20 1 9,

arXiv : 1 906 .08237 .  2

[ZK 1 6]  Sergey  Zagoruyko  and  Nikos  Komodakis .  Wide  residual  networks .  Procedings  of  the B  ritish

Machine  Vision  Conference  201 6,  20 1 6 .  doi : 1 0. 5244/c . 30. 87 .   1 8

[ZKZ+ 1 5]  Yukun  Zhu,  Ryan  Kiros,  Rich  Zemel,  Ruslan  Salakhutdinov,  Raquel  Urtasun,  Antonio  Tor

ralba,  and  S anj a  Fidler.  Aligning  books  and  movies :  Towards  story-like  visual  explanations  by

watching  movies  and  reading  books .  2015 I  EEE I  nternational  Conference  on  Computer  Vision

(ICCV) ,  Dec  20 1 5 .  doi : 1 0 . 1 1 09/iccv. 20 1 5 . 1 1 .  7

[ZLN+ 1 9]  Guodong  Zhang,  Lala  Li,  Zachary  Nado,  James  Martens,  Sushant  Sachdeva,  George  E.  Dahl,

Christopher  J.  Shallue,  and  Roger  B .  Grosse.  Which  algorithmic  choices  matter  at  which  batch

sizes ?  insights  from  a  noisy  quadratic  model.  CoRR,  abs/ 1 907 .04 1 64,  20 1 9,   1 907 .04 1 64.  URL

http : / / arx iv . org/ ab s / 1 907 . 04 1 64 .   1 2,   1 8

30
