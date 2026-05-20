Provided  proper  attribution  is  provided,  Google  hereby  grants  permission  to

reproduce  the  tables  and  figures  in  this  paper  solely  for  use  in j  ournalistic  or

scholarly  works .

Attention  Is  All  You  Need

3

2

0 Ashish  Vaswani∗
 Noam  Shazeer∗
 Niki  Parmar∗
 Jakob  Uszkoreit∗

2 Google  Brain
 Google  Brain
 Google  Research
 Google  Research

g avaswani@google . com
 noam@google . com
 nikip@google . com
 usz@google . com

u

A Llion  Jones∗
 Aidan  N.  Gomez∗  †
 Łukasz  Kaiser∗

Google  Research
 University  of  Toronto
 Google  Brain

2

l l i on@go ogl e . c om
 aidan@ c s . t oront o . edu
 lukaszkai s er@go ogl e . c om

]

L Illia  Polosukhin∗  ‡

C. i ll i a . polo sukhin@gmai l . com

s

c

[

7 Abstract

v

2 The  dominant  sequence  transduction  models  are  based  on  complex  recurrent  or

6 convolutional  neural  networks  that  include  an  encoder  and  a  decoder.  The  best

7

performing  models  also  connect  the  encoder  and  decoder  through  an  attention

3

mechanism.  We  propose  a  new  simple  network  architecture,  the  Transformer,

0

. based  solely  on  attention  mechanisms,  dispensing  with  recurrence  and  convolutions

6 entirely.  Experiments  on  two  machine  translation  tasks  show  these  models  to

0 be  superior  in  quality  while  being  more  parallelizable  and  requiring  significantly

7 less  time  to  train.  Our  model  achieves  28 .4  BLEU  on  the  WMT  20 1 4  English

1

: to-German  translation  task,  improving  over  the  existing  best  results,  including

v ensembles,  by  over  2  BLEU.  On  the  WMT  20 1 4  English-to-French  translation  task,

i our  model  establishes  a  new  single-model  state-of-the-art  BLEU  score  of  4 1 . 8  after

X training  for  3 . 5  days  on  eight  GPUs ,  a  small  fraction  of  the  training  costs  of  the

r

a best  models  from  the  literature.  We  show  that  the  Transformer  generalizes  well  to

other  tasks  by  applying  it  successfully  to  English  constituency  parsing  both  with

large  and  limited  training  data.

∗ Equal  contribution.  Listing  order  is  random.  Jakob  proposed  replacing  RNNs  with  self-attention  and  started

the  effort  to  evaluate  this  idea.  Ashish,  with  Illia,  designed  and  implemented  the  first  Transformer  models  and

has  been  crucially  involved  in  every  aspect  of  this  work.  Noam  proposed  scaled  dot-product  attention,  multi-head

attention  and  the  parameter-free  position  representation  and  became  the  other  person  involved  in  nearly  every

detail.  Niki  designed,  implemented,  tuned  and  evaluated  countless  model  variants  in  our  original  codebase  and

tensor2tensor.  Llion  also  experimented  with  novel  model  variants,  was  responsible  for  our  initial  codebase,  and

efficient  inference  and  visualizations .  Lukasz  and  Aidan  spent  countless  long  days  designing  various  parts  of  and

implementing  tensor2tensor,  replacing  our  earlier  codebase,  greatly  improving  results  and  massively  accelerating

our  research.

† Work  performed  while  at  Google  Brain.

‡ Work  performed  while  at  Google  Research.

3 1 st  Conference  on  Neural  Information  Processing  Systems  (NIPS  20 1 7),  Long  Beach,  CA,  USA.

1  Introduction

Recurrent  neural  networks,  long  short-term  memory  [ 1 3]  and  gated  recurrent  [7]  neural  networks

in  particular,  have  been  firmly  established  as  state  of  the  art  approaches  in  sequence  modeling  and

transduction  problems  such  as  language  modeling  and  machine  translation  [35 ,  2,  5] .  Numerous

efforts  have  since  continued  to  push  the  boundaries  of  recurrent  language  models  and  encoder-decoder

architectures   [3 8 ,  24 ,   1 5 ] .

Recurrent  models  typically  factor  computation  along  the  symbol  positions  of  the  input  and  output

sequences .  Aligning  the  positions  to  steps  in  computation  time,  they  generate  a  sequence  of  hidden

states  ht ,  as  a  function  of  the  previous  hidden  state  ht − 1  and  the  input  for  position  t .  This  inherently

sequential  nature  precludes  parallelization  within  training  examples,  which  becomes  critical  at  longer

sequence  lengths,  as  memory  constraints  limit  batching  across  examples .  Recent  work  has  achieved

significant  improvements  in  computational  efficiency  through  factorization  tricks  [2 1 ]  and  conditional

computation  [32] ,  while  also  improving  model  performance  in  case  of  the  latter.  The  fundamental

constraint  of  sequential  computation,  however,  remains .

Attention  mechanisms  have  become  an  integral  part  of  compelling  sequence  modeling  and  transduc

tion  models  in  various  tasks,  allowing  modeling  of  dependencies  without  regard  to  their  distance  in

the  input  or  output  sequences  [2,  1 9] .  In  all  but  a  few  cases  [27] ,  however,  such  attention  mechanisms

are  used  in  conjunction  with  a  recurrent  network.

In  this  work  we  propose  the  Transformer,  a  model  architecture  eschewing  recurrence  and  instead

relying  entirely  on  an  attention  mechanism  to  draw  global  dependencies  between  input  and  output.

The  Transformer  allows  for  significantly  more  parallelization  and  can  reach  a  new  state  of  the  art  in

translation  quality  after  being  trained  for  as  little  as  twelve  hours  on  eight  P 1 00  GPUs .

2  Background

The  goal  of  reducing  sequential  computation  also  forms  the  foundation  of  the  Extended  Neural  GPU

[ 1 6] ,  ByteNet  [ 1 8]  and  ConvS2S  [9] ,  all  of  which  use  convolutional  neural  networks  as  basic  building

block,  computing  hidden  representations  in  parallel  for  all  input  and  output  positions .  In  these  models ,

the  number  of  operations  required  to  relate  signals  from  two  arbitrary  input  or  output  positions  grows

in  the  distance  between  positions,  linearly  for  ConvS2S  and  logarithmically  for  ByteNet.  This  makes

it  more  difficult  to  learn  dependencies  between  distant  positions  [ 1 2] .  In  the  Transformer  this  is

reduced  to  a  constant  number  of  operations ,  albeit  at  the  cost  of  reduced  effective  resolution  due

to  averaging  attention-weighted  positions,  an  effect  we  counteract  with  Multi-Head  Attention  as

described  in  section  3 . 2 .

Self-attention,  sometimes  called  intra-attention  is  an  attention  mechanism  relating  different  positions

of  a  single  sequence  in  order  to  compute  a  representation  of  the  sequence.  Self-attention  has  been

used  successfully  in  a  variety  of  tasks  including  reading  comprehension,  abstractive  summarization,

textual  entailment  and  learning  task-independent  sentence  representations  [4,  27 ,  28 ,  22] .

End-to-end  memory  networks  are  based  on  a  recurrent  attention  mechanism  instead  of  sequence

aligned  recurrence  and  have  been  shown  to  perform  well  on  simple-language  question  answering  and

language  modeling  tasks  [34] .

To  the  best  of  our  knowledge,  however,  the  Transformer  is  the  first  transduction  model  relying

entirely  on  self-attention  to  compute  representations  of  its  input  and  output  without  using  sequence

aligned  RNNs  or  convolution.  In  the  following  sections,  we  will  describe  the  Transformer,  motivate

self-attention  and  discus s  its  advantages  over  models  such  as  [ 1 7 ,   1 8]  and  [9] .

# 3  Model  Architecture

Most  competitive  neural  sequence  transduction  models  have  an  encoder-decoder  structure  [5 ,  2,  3 5] .

Here,  the  encoder  maps  an  input  sequence  of  symbol  representations  (x 1 ,  . . . ,  xn )   to  a  sequence

of  continuous  representations  z  =   ( z1 ,  . . . ,  zn ) .  Given  z ,  the  decoder  then  generates  an  output

sequence  (y1 ,  . . . ,  ym )   of  symbols  one  element  at  a  time.  At  each  step  the  model  is  auto-regres sive

[ 1 0] ,  consuming  the  previously  generated  symbols  as  additional  input  when  generating  the  next.

2

Figure   1 :  The  Transformer  -  model  architecture.

The  Transformer  follows  this  overall  architecture  using  stacked  self-attention  and  point-wise,  fully

connected  layers  for  both  the  encoder  and  decoder,  shown  in  the  left  and  right  halves  of  Figure   1 ,

respectively.

3.1  Encoder  and  Decoder  Stacks

Encoder:  The  encoder  is  composed  of  a  stack  of  N  =  6  identical  layers .  Each  layer  has  two

sub-layers .  The  first  is  a  multi-head  self-attention  mechanism,  and  the  second  is  a  simple,  position

wise  fully  connected  feed-forward  network.  We  employ  a  residual  connection  [ 1 1 ]  around  each  of

the  two  sub-layers ,  followed  by  layer  normalization  [ 1 ] .  That  is ,  the  output  of  each  sub-layer  is

LayerNorm (x  +  Sublayer (x) ) ,  where  Sublayer (x)  is  the  function  implemented  by  the  sub-layer

itself.  To  facilitate  these  residual  connections ,  all  sub-layers  in  the  model,  as  well  as  the  embedding

layers ,  produce  outputs  of  dimension  dmodel  =  5 1 2 .

Decoder:  The  decoder  is  also  composed  of  a  stack  of  N  =  6  identical  layers .  In  addition  to  the  two

sub-layers  in  each  encoder  layer,  the  decoder  inserts  a  third  sub-layer,  which  performs  multi-head

attention  over  the  output  of  the  encoder  stack.  Similar  to  the  encoder,  we  employ  residual  connections

around  each  of  the  sub-layers,  followed  by  layer  normalization.  We  also  modify  the  self-attention

sub-layer  in  the  decoder  stack  to  prevent  positions  from  attending  to  subsequent  positions .  This

masking,  combined  with  fact  that  the  output  embeddings  are  offset  by  one  position,  ensures  that  the

predictions  for  position  i  can  depend  only  on  the  known  outputs  at  positions  less  than  i .

3.2  Attention

An  attention  function  can  be  described  as  mapping  a  query  and  a  set  of  key-value  pairs  to  an  output,

where  the  query,  keys ,  values ,  and  output  are  all  vectors .  The  output  is  computed  as  a  weighted  sum

3

Scaled  Dot-Product  Attention  Multi-Head  Attention

Figure  2 :  (left)  Scaled  Dot-Product  Attention.  (right)  Multi-Head  Attention  consists  of  several

attention  layers  running  in  parallel .

of  the  values ,  where  the  weight  assigned  to  each  value  is  computed  by  a  compatibility  function  of  the

query  with  the  corresponding  key.

3.2.1  Scaled  Dot-Product  Attention

We  call  our  particular  attention  " Scaled  Dot-Product  Attention"  (Figure  2) .  The  input  consists  of

queries  and  keys  of  dimension  dk ,  and
  values  of  dimension  dv .  We  compute  the  dot  products  of  the

query  with  all  keys ,  divide  each  by  √dk ,  and  apply  a  softmax  function  to  obtain  the  weights  on  the

values .

In  practice,  we  compute  the  attention  function  on  a  set  of  queries  simultaneously,  packed  together

into  a  matrix  Q .  The  keys  and  values  are  also  packed  together  into  matrices  K  and  V  .  We  compute

the  matrix  of  outputs  as :

KT

Attention ( Q ,  K,  V  )  =  softmax ( Q 
 ) V  ( 1 )

√dk

The  two  most  commonly  used  attention  functions  are  additive  attention  [2] ,  and  dot-product  (multi

plicative)  attention.  Dot-product  attention  is  identical  to  our  algorithm,  except  for  the  scaling  factor

of  1
 
 .  Additive  attention  computes  the  compatibility  function  using  a  feed-forward  network  with

√d k

a  single  hidden  layer.  While  the  two  are  similar  in  theoretical  complexity,  dot-product  attention  is

much  faster  and  more  space-efficient  in  practice,  since  it  can  be  implemented  using  highly  optimized

matrix  multiplication  code.

While  for  small  values  of  dk  the  two  mechanisms  perform  similarly,  additive  attention  outperforms

dot  product  attention  without  scaling  for  larger  values  of  dk  [3 ] .  We  suspect  that  for  large  values  of

dk ,  the  dot  products  grow  large  in  magnitude,  pushing  the  softmax  function  into  regions  where  it  has

extremely  small  gradients  4
 .  To  counteract  this  effect,  we  scale  the  dot  products  by  1
 
 .

√d k

3.2.2  Multi-Head  Attention

Instead  of  performing  a  single  attention  function  with  dmodel -dimensional  keys,  values  and  queries,

we  found  it  beneficial  to  linearly  proj ect  the  queries ,  keys  and  values  h  times  with  different,  learned

linear  proj ections  to  dk ,  dk  and  dv  dimensions ,  respectively.  On  each  of  these  proj ected  versions  of

queries,  keys  and  values  we  then  perform  the  attention  function  in  parallel,  yielding  dv -dimensional

4To  illustrate  why  the  dot  products  get  large,  assume  that  the  components  of  q  and  k  are  independent  random

variables  with  mean  0  and  variance  1 .  Then  their  dot  product,  q  ·  k  =
 P id=k
 1  qi ki ,  has  mean  0  and  variance  dk .

4

output  values .  These  are  concatenated  and  once  again  proj ected,  resulting  in  the  final  values ,  as

depicted  in  Figure  2 .

Multi-head  attention  allows  the  model  to j  ointly  attend  to  information  from  different  representation

subspaces  at  different  positions .  With  a  single  attention  head,  averaging  inhibits  this .

MultiHead ( Q ,  K,  V  )  =  Concat (head 1 ,  . . . ,  headh ) WO

where  headi  =  Attent ion ( Q WiQ
 
 ,  KW iK
 
 ,  V WiV
 
 )

Where  the  proj ections  are  parameter  matrices  W
iQ  
 ∈   R
dmodel × dk  ,  W iK  
 ∈   R
dmodel × dk  ,  WiV  
 ∈   R
dmodel × dv

and  W O  ∈   R
hdv × dmodel
 .

In  this  work  we  employ  h  =  8  parallel  attention  layers ,  or  heads .  For  each  of  these  we  use

dk  =  dv  =  dmodel / h  =  64 .  Due  to  the  reduced  dimension  of  each  head,  the  total  computational  cost

is  similar  to  that  of  single-head  attention  with  full  dimensionality.

3.2.3  Applications  of  Attention  in  our  Model

The  Transformer  uses  multi-head  attention  in  three  different  ways :

•  In  " encoder-decoder  attention"  layers,  the  queries  come  from  the  previous  decoder  layer,

and  the  memory  keys  and  values  come  from  the  output  of  the  encoder.  This  allows  every

position  in  the  decoder  to  attend  over  all  positions  in  the  input  sequence.  This  mimics  the

typical  encoder-decoder  attention  mechanisms  in  sequence-to-sequence  models  such  as

[ 3 8 ,  2 ,  9 ] .

•  The  encoder  contains  self-attention  layers .  In  a  self-attention  layer  all  of  the  keys ,  values

and  queries  come  from  the  same  place,  in  this  case,  the  output  of  the  previous  layer  in  the

encoder.  Each  position  in  the  encoder  can  attend  to  all  positions  in  the  previous  layer  of  the

encoder.

•  Similarly,  self-attention  layers  in  the  decoder  allow  each  position  in  the  decoder  to  attend  to

all  positions  in  the  decoder  up  to  and  including  that  position.  We  need  to  prevent  leftward

information  flow  in  the  decoder  to  preserve  the  auto-regressive  property.  We  implement  this

inside  of  scaled  dot-product  attention  by  masking  out  (setting  to  − ∞)  all  values  in  the  input

of  the  softmax  which  correspond  to  illegal  connections .  See  Figure  2.

3.3  Position-wise  Feed-Forward  Networks

In  addition  to  attention  sub-layers ,  each  of  the  layers  in  our  encoder  and  decoder  contains  a  fully

connected  feed-forward  network,  which  is  applied  to  each  position  separately  and  identically.  This

consists  of  two  linear  transformations  with  a  ReLU  activation  in  between.

FFN (x )  =  max (0 ,  xW1  +  b1 ) W2  +  b2  (2)

While  the  linear  transformations  are  the  same  across  different  positions,  they  use  different  parameters

from  layer  to  layer.  Another  way  of  describing  this  is  as  two  convolutions  with  kernel  size   1 .

The  dimensionality  of  input  and  output  is  dmodel  =  5 1 2 ,  and  the  inner-layer  has  dimensionality

df f  =  2 048 .

3.4  Embeddings  and  Softmax

Similarly  to  other  sequence  transduction  models,  we  use  learned  embeddings  to  convert  the  input

tokens  and  output  tokens  to  vectors  of  dimension  dmodel .  We  also  use  the  usual  learned  linear  transfor

mation  and  softmax  function  to  convert  the  decoder  output  to  predicted  next-token  probabilities .  In

our  model,  we  share  the  same  weight  matrix  between  the  two  embedding  layers  and  the  pre-so
 ftmax

linear  transformation,  similar  to  [30] .  In  the  embedding  layers ,  we  multiply  those  weights  by  √dmodel .

5

Table   1 :  Maximum  path  lengths,  per-layer  complexity  and  minimum  number  of  sequential  operations

for  different  layer  types .  n  is  the  sequence  length,  d  is  the  representation  dimension,  k  is  the  kernel

size  of  convolutions  and  r  the  size  of  the  neighborhood  in  restricted  self-attention.

Layer  Type  Complexity  per  Layer  Sequential  Maximum  Path  Length

Operations

S elf-Attention  O ( n
2
 ·   d)   O ( 1 )   O ( 1 )

Recurrent  O ( n  ·   d
2
 )   O ( n )   O ( n )

Convolutional  O ( k  ·   n  ·   d
2
 )   O ( 1 )   O ( l ogk ( n ) )

S elf-Attention  (restricted)  O (r  ·  n  ·  d)   O ( 1 )   O (n/r )

3.5  Positional  Encoding

Since  our  model  contains  no  recurrence  and  no  convolution,  in  order  for  the  model  to  make  use  of  the

order  of  the  sequence,  we  must  inj ect  some  information  about  the  relative  or  absolute  position  of  the

" "

tokens  in  the  sequence.  To  this  end,  we  add   positional  encodings  to  the  input  embeddings  at  the

bottoms  of  the  encoder  and  decoder  stacks .  The  positional  encodings  have  the  same  dimension  dmodel

as  the  embeddings,  so  that  the  two  can  be  summed.  There  are  many  choices  of  positional  encodings,

learned  and  fixed  [9] .

In  this  work,  we  use  sine  and  cosine  functions  of  different  frequencies :

P E(pos , 2i)  =  sin (pos / 1 00002i / dmodel )

P E(pos , 2i+ 1 )  =  cos (pos / 1 00002i/ dmodel )

where  pos  is  the  position  and  i  is  the  dimension.  That  is ,  each  dimension  of  the  positional  encoding

corresponds  to  a  sinusoid.  The  wavelengths  form  a  geometric  progression  from  2π  to  1 0000  ·  2π .  We

chose  this  function  because  we  hypothesized  it  would  allow  the  model  to  easily  learn  to  attend  by

relative  positions ,  since  for  any  fixed  offset  k ,  P Epos + k  can  be  represented  as  a  linear  function  of

P Epos .

We  also  experimented  with  using  learned  positional  embeddings  [9]  instead,  and  found  that  the  two

versions  produced  nearly  identical  results  (see  Table  3  row  (E)) .  We  chose  the  sinusoidal  version

because  it  may  allow  the  model  to  extrapolate  to  sequence  lengths  longer  than  the  ones  encountered

during  training .

# 4  Why  Self-Attention

In  this  section  we  compare  various  aspects  of  self-attention  layers  to  the  recurrent  and  convolu

tional  layers  commonly  used  for  mapping  one  variable-length  sequence  of  symbol  representations

( x 1 ,   . . . ,  x n )   to  another  sequence  of  equal  length  ( z1 ,   . . . ,  zn ) ,  with  xi
 ,  zi  ∈   R
d
 ,  such  as  a  hidden

layer  in  a  typical  sequence  transduction  encoder  or  decoder.  Motivating  our  use  of  self-attention  we

consider  three  desiderata.

One  is  the  total  computational  complexity  per  layer.  Another  is  the  amount  of  computation  that  can

be  parallelized,  as  measured  by  the  minimum  number  of  sequential  operations  required.

The  third  is  the  path  length  between  long-range  dependencies  in  the  network.  Learning  long-range

dependencies  is  a  key  challenge  in  many  sequence  transduction  tasks .  One  key  factor  affecting  the

ability  to  learn  such  dependencies  is  the  length  of  the  paths  forward  and  backward  signals  have  to

traverse  in  the  network.  The  shorter  these  paths  between  any  combination  of  positions  in  the  input

and  output  sequences ,  the  easier  it  is  to  learn  long-range  dependencies  [ 1 2] .  Hence  we  also  compare

the  maximum  path  length  between  any  two  input  and  output  positions  in  networks  composed  of  the

different  layer  types .

As  noted  in  Table   1 ,  a  self-attention  layer  connects  all  positions  with  a  constant  number  of  sequentially

executed  operations,  whereas  a  recurrent  layer  requires  O (n)  sequential  operations .  In  terms  of

computational  complexity,  self-attention  layers  are  faster  than  recurrent  layers  when  the  sequence

6

length  n  is  smaller  than  the  representation  dimensionality  d,  which  is  most  often  the  case  with

sentence  representations  used  by  state-of-the-art  models  in  machine  translations,  such  as  word-piece

[3 8]  and  byte-pair  [3 1 ]  representations .  To  improve  computational  performance  for  tasks  involving

very  long  sequences ,  self-attention  could  be  restricted  to  considering  only  a  neighborhood  of  size  r  in

the  input  sequence  centered  around  the  respective  output  position.  This  would  increase  the  maximum

path  length  to  O (n/r ) .  We  plan  to  investigate  this  approach  further  in  future  work.

A  single  convolutional  layer  with  kernel  width  k  <  n  does  not  connect  all  pairs  of  input  and  output

positions .  Doing  so  requires  a  stack  of  O (n/ k )   convolutional  layers  in  the  case  of  contiguous  kernels ,

or  O ( l ogk (n) )   in  the  case  of  dilated  convolutions  [ 1 8] ,  increasing  the  length  of  the  longest  paths

between  any  two  positions  in  the  network.  Convolutional  layers  are  generally  more  expensive  than

recurrent  layers ,  by  a  factor  of  k .  Separable  convolutions  [6] ,  however,  decrease  the  complexity

considerably,  to  O ( k  ·  n  ·  d  +  n  ·  d
2
 ) .  Even  with  k  =  n,  however,  the  complexity  of  a  separable

convolution  is  equal  to  the  combination  of  a  self-attention  layer  and  a  point-wise  feed-forward  layer,

the  approach  we  take  in  our  model.

As  side  benefit,  self-attention  could  yield  more  interpretable  models .  We  inspect  attention  distributions

from  our  models  and  present  and  discuss  examples  in  the  appendix.  Not  only  do  individual  attention

heads  clearly  learn  to  perform  different  tasks ,  many  appear  to  exhibit  behavior  related  to  the  syntactic

and  semantic  structure  of  the  sentences .

5  Training

This  section  describes  the  training  regime  for  our  models .

5.1  Training  Data  and  Batching

We  trained  on  the  standard  WMT  20 1 4  English-German  dataset  consisting  of  about  4.5  million

sentence  pairs .  Sentences  were  encoded  using  byte-pair  encoding  [3 ] ,  which  has  a  shared  source

target  vocabulary  of  about  37000  tokens .  For  English-French,  we  used  the  significantly  larger  WMT

20 1 4  English-French  dataset  consisting  of  3 6M  sentences  and  split  tokens  into  a  32000  word-piece

vocabulary  [3 8] .  Sentence  pairs  were  batched  together  by  approximate  sequence  length.  Each  training

batch  contained  a  set  of  sentence  pairs  containing  approximately  25000  source  tokens  and  25000

target  tokens .

5.2  Hardware  and  Schedule

We  trained  our  models  on  one  machine  with  8  NVIDIA  P 1 00  GPUs .  For  our  base  models  using

the  hyperparameters  described  throughout  the  paper,  each  training  step  took  about  0.4  seconds .  We

trained  the  base  models  for  a  total  of   1 00,000  steps  or   1 2  hours .  For  our  big  models , (described  on  the

bottom  line  of  table  3 ) ,  step  time  was   1 .0  seconds .  The  big  models  were  trained  for  300,000  steps

(3 . 5  day s) .

5.3  Optimizer

We  used  the  Adam  optimizer  [20]  with  β1  =  0 . 9 ,  β2  =  0 . 98  and  ϵ  =  1 0 − 9
 .  We  varied  the  learning

rate  over  the  course  of  training,  according  to  the  formula:

= − 0 . 5
 · − 0 . 5
 · − 1 . 5

lrate    d
model    min ( step_num ,  step_num    warmup_steps )  (3)

This  corresponds  to  increasing  the  learning  rate  linearly  for  the  first  warmup_steps  training  steps,

and  decreasing  it  thereafter  proportionally  to  the  inverse  square  root  of  the  step  number.  We  used

warmup_steps  =  4000.

5.4  Regularization

We  employ  three  types  of  regularization  during  training :

7

Table  2 :  The  Transformer  achieves  better  BLEU  scores  than  previous  state-of-the-art  models  on  the

English-to-German  and  English-to-French  newstest20 1 4  tests  at  a  fraction  of  the  training  cost.

Model

BLEU  Training  Cost  (FLOPs)

EN-DE  EN-FR  EN-DE  EN-FR

ByteNet  [ 1 8]  23 .75

Deep-Att  +  PosUnk  [3 9]  3 9 . 2  1 . 0  ·  1 020

GNMT  +  RL   [3 8 ]  24 . 6  3 9 . 92  2 . 3  ·   1 0 1 9  1 . 4  ·   1 0 20

ConvS 2S   [9]  25 . 1 6  40 . 46  9 . 6  ·   1 0 1 8  1 . 5  ·   1 0 2 0

MoE   [3 2]  26 . 03  40 . 5 6  2 . 0  ·   1 0 1 9  1 . 2  ·   1 0 2 0

Deep-Att  +  PosUnk  Ensemble  [3 9]  40.4  8 . 0  ·  1 020

GNMT  +  RL  Ensemble   [3 8 ]  26 . 3 0  4 1 . 1 6  1 . 8  ·  1 020  1 . 1  ·  1 02 1

ConvS 2S  Ensemble   [9]  26 . 3 6  4 1 .29  7 . 7  ·  1 0 1 9  1 . 2  ·  1 02 1

Transformer  (base  model)  27 . 3  3 8 . 1   3 . 3  ·  1 0 1 8

Transformer  (big)  28.4  4 1 .8  2 . 3  ·  1 0 1 9

Residual  Dropout  We  apply  dropout  [3 3 ]  to  the  output  of  each  sub-layer,  before  it  is  added  to  the

sub-layer  input  and  normalized.  In  addition,  we  apply  dropout  to  the  sums  of  the  embeddings  and  the

positional  encodings  in  both  the  encoder  and  decoder  stacks .  For  the  base  model,  we  use  a  rate  of

Pdrop  =  0 . 1 .

Label  Smoothing  During  training,  we  employed  label  smoothing  of  value  ϵl s  =  0 . 1  [3 6] .  This

hurts  perplexity,  as  the  model  learns  to  be  more  unsure,  but  improves  accuracy  and  BLEU  score.

6  Results

6.1  Machine  Translation

On  the  WMT  20 1 4  English-to-German  translation  task,  the  big  transformer  model  (Transformer  (big)

in  Table  2)  outperforms  the  best  previously  reported  models  (including  ensembles)  by  more  than  2 . 0

BLEU,  establishing  a  new  state-of-the-art  BLEU  score  of  28 . 4.  The  configuration  of  this  model  is

listed  in  the  bottom  line  of  Table  3 .  Training  took  3 . 5  days  on  8  P 1 00  GPUs .  Even  our  base  model

surpasses  all  previously  published  models  and  ensembles ,  at  a  fraction  of  the  training  cost  of  any  of

the  competitive  models .

On  the  WMT  20 1 4  English-to-French  translation  task,  our  big  model  achieves  a  BLEU  score  of  4 1 . 0,

outperforming  all  of  the  previously  published  single  models ,  at  less  than  1 /4  the  training  cost  of  the

previous  state-of-the-art  model.  The  Transformer  (big)  model  trained  for  English-to-French  used

dropout  rate  Pdrop  =  0 . 1 ,  instead  of  0 . 3 .

For  the  base  models,  we  used  a  single  model  obtained  by  averaging  the  last  5  checkpoints,  which

were  written  at   1 0-minute  intervals .  For  the  big  models ,  we  averaged  the  last  20  checkpoints .  We

used  beam  search  with  a  beam  size  of  4  and  length  penalty  α  =  0 . 6  [3 8] .  These  hyperparameters

were  chosen  after  experimentation  on  the  development  set.  We  set  the  maximum  output  length  during

inference  to  input  length  +  50 ,  but  terminate  early  when  possible  [3 8] .

Table  2  summarizes  our  results  and  compares  our  translation  quality  and  training  costs  to  other  model

architectures  from  the  literature.  We  estimate  the  number  of  floating  point  operations  used  to  train  a

model  by  multiplying  the  training  time,  the  number  of  GPUs  used,  and  an  estimate  of  the  sustained

single-precision  floating-point  capacity  of  each  GPU  5
 .

6.2  Model  Variations

To  evaluate  the  importance  of  different  components  of  the  Transformer,  we  varied  our  base  model

in  different  ways,  measuring  the  change  in  performance  on  English-to-German  translation  on  the

5We  used  values  of  2. 8 ,  3 .7 ,  6 .0  and  9 . 5  TFLOPS  for  K80,  K40,  M40  and  P 1 00,  respectively.

8

Table  3 :  Variations  on  the  Transformer  architecture.  Unlisted  values  are  identical  to  those  of  the  base

model.  All  metrics  are  on  the  English-to-German  translation  development  set,  newstest20 1 3 .  Listed

perplexities  are  per-wordpiece,  according  to  our  byte-pair  encoding,  and  should  not  be  compared  to

per-word  perplexities .

train  PPL  BLEU  params

N  dmodel  dff  h  dk  dv  Pdrop  ϵl s
 6

steps  (dev)  (dev)  × 1 0

base  6  5 1 2  204 8  8  64  64  0 . 1  0 . 1   1 00K  4 . 92  25 . 8  65

(A)

1  5 1 2  5 1 2  5 . 29  24 . 9

4   1 2 8   1 2 8  5 . 00  25 . 5

1 6  3 2  3 2  4 . 9 1  25 . 8

3 2   1 6   1 6  5 . 0 1  25 . 4

(B )

1 6  5 . 1 6  25 . 1  5 8

3 2  5 . 0 1  25 . 4  60

2  6 . 1 1  23 . 7  3 6

4  5 . 1 9  25 . 3  5 0

8  4 . 8 8  25 . 5  8 0

(C)
 25 6  3 2  3 2  5 . 7 5  24 . 5  2 8

1 024   1 28   1 28  4 . 66  26 . 0   1 6 8

1 024  5 . 1 2  25 . 4  5 3

4096  4 . 75  26 . 2  90

(D)

0 . 0  5 . 77  24 . 6

0 . 2  4 . 95  25 . 5

0 . 0  4 . 67  25 . 3

0 . 2  5 . 47  25 . 7

(E)  positional  embedding  instead  of  sinusoids  4 . 92  25 .7

big  6   1 024  4096   1 6  0 . 3  3 00K  4.33  26.4  2 1 3

development  set,  newstest20 1 3 .  We  used  beam  search  as  described  in  the  previous  section,  but  no

checkpoint  averaging .  We  present  these  results  in  Table  3 .

In  Table  3  rows  (A) ,  we  vary  the  number  of  attention  heads  and  the  attention  key  and  value  dimensions,

keeping  the  amount  of  computation  constant,  as  described  in  Section  3 . 2. 2.  While  single-head

attention  is  0. 9  BLEU  worse  than  the  best  setting,  quality  also  drops  off  with  too  many  heads .

In  Table  3  rows  (B ) ,  we  observe  that  reducing  the  attention  key  size  dk  hurts  model  quality.  This

suggests  that  determining  compatibility  is  not  easy  and  that  a  more  sophisticated  compatibility

function  than  dot  product  may  be  beneficial.  We  further  observe  in  rows  (C)  and  (D)  that,  as  expected,

bigger  models  are  better,  and  dropout  is  very  helpful  in  avoiding  over-fitting .  In  row  (E)  we  replace  our

sinusoidal  positional  encoding  with  learned  positional  embeddings  [9] ,  and  observe  nearly  identical

results  to  the  base  model .

6.3  English  Constituency  Parsing

To  evaluate  if  the  Transformer  can  generalize  to  other  tasks  we  performed  experiments  on  English

constituency  parsing .  This  task  presents  specific  challenges :  the  output  is  subj ect  to  strong  structural

constraints  and  is  significantly  longer  than  the  input.  Furthermore,  RNN  sequence-to-sequence

models  have  not  been  able  to  attain  state-of-the-art  results  in  small-data  regimes  [37] .

We  trained  a  4-layer  transformer  with  dmodel  =  1 024  on  the  Wall  Street  Journal  (WSJ)  portion  of  the

Penn  Treebank  [25] ,  about  40K  training  sentences .  We  also  trained  it  in  a  semi- supervised  setting,

using  the  larger  high-confidence  and  BerkleyParser  corpora  from  with  approximately   1 7M  sentences

[37] .  We  used  a  vocabulary  of   1 6K  tokens  for  the  WSJ  only  setting  and  a  vocabulary  of  3 2K  tokens

for  the  semi- supervised  setting .

We  performed  only  a  small  number  of  experiments  to  select  the  dropout,  both  attention  and  residual

(section  5 .4) ,  learning  rates  and  beam  size  on  the  Section  22  development  set,  all  other  parameters

remained  unchanged  from  the  English-to-German  base  translation  model.  During  inference,  we

9

Table  4 :  The  Transformer  generalizes  well  to  English  constituency  parsing  (Results  are  on  Section  23

of  WSJ)

Parser  Training  WSJ  23  F1

Vinyals  &  Kaiser  el  al.  (20 1 4)  [37]  WSJ  only,  discriminative  8 8 . 3

Petrov  et  al.  (2006)  [29]  WSJ  only,  discriminative  90 .4

Zhu  et  al.  (20 1 3 )  [40]  WSJ  only,  discriminative  90 .4

Dyer  et  al .  (20 1 6)  [ 8]  WSJ  only,  discriminative  9 1 .7

Transformer  (4  layers)  WSJ  only,  discriminative  9 1 . 3

Zhu  et  al .  (20 1 3 )  [40]  semi- supervised  9 1 . 3

Huang  &  Harper  (2009)  [ 1 4]  semi-supervised  9 1 . 3

McClosky  et  al.  (2006)  [26]  semi- supervised  92 . 1

Vinyals  &  Kaiser  el  al.  (20 1 4)  [37]  semi- supervised  92 . 1

Transformer  (4  layers)  semi-supervised  92.7

Luong  et  al .  (20 1 5 )   [23 ]   multi-task  93 . 0

Dyer  et  al .  (20 1 6)   [ 8 ]   generative  93 . 3

increased  the  maximum  output  length  to  input  length  +  300 .  We  used  a  beam  size  of  2 1  and  α  =  0 . 3

for  both  WSJ  only  and  the  semi- supervised  setting .

Our  results  in  Table  4  show  that  despite  the  lack  of  task- specific  tuning  our  model  performs  sur

prisingly  well,  yielding  better  results  than  all  previously  reported  models  with  the  exception  of  the

Recurrent  Neural  Network  Grammar  [8] .

In  contrast  to  RNN  sequence-to-sequence  models  [37] ,  the  Transformer  outperforms  the  Berkeley

Parser  [29]  even  when  training  only  on  the  WSJ  training  set  of  40K  sentences .

7  Conclusion

In  this  work,  we  presented  the  Transformer,  the  first  sequence  transduction  model  based  entirely  on

attention,  replacing  the  recurrent  layers  most  commonly  used  in  encoder-decoder  architectures  with

multi-headed  self-attention.

For  translation  tasks,  the  Transformer  can  be  trained  significantly  faster  than  architectures  based

on  recurrent  or  convolutional  layers .  On  both  WMT  20 1 4  English-to-German  and  WMT  20 1 4

English-to-French  translation  tasks ,  we  achieve  a  new  state  of  the  art.  In  the  former  task  our  best

model  outperforms  even  all  previously  reported  ensembles .

We  are  excited  about  the  future  of  attention-based  models  and  plan  to  apply  them  to  other  tasks .  We

plan  to  extend  the  Transformer  to  problems  involving  input  and  output  modalities  other  than  text  and

to  investigate  local,  restricted  attention  mechanisms  to  efficiently  handle  large  inputs  and  outputs

such  as  images ,  audio  and  video .  Making  generation  less  sequential  is  another  research  goals  of  ours .

The  code  we  used  to  train  and  evaluate  our  models  is  available  at  http s : / /github . c om/

t ens orf low/t ens or2t ens or .

Acknowledgements  We  are  grateful  to  Nal  Kalchbrenner  and  Stephan  Gouws  for  their  fruitful

comments,  corrections  and  inspiration.

References

[ 1 ]  Jimmy  Lei  B a,  Jamie  Ryan  Kiros,  and  Geoffrey  E  Hinton.  Layer  normalization.  arXiv p  reprint

arXiv: 1 607. 06450,  20 1 6 .

[2]  Dzmitry  B ahdanau,  Kyunghyun  Cho,  and  Yoshua  Bengio.  Neural  machine  translation  by j  ointly

learning  to  align  and  translate.  CoRR,  abs/ 1 409 .0473 ,  20 1 4 .

[3 ]  Denny  Britz,  Anna  Goldie,  Minh-Thang  Luong,  and  Quoc  V.  Le.  Massive  exploration  of  neural

machine  translation  architectures .  CoRR,  abs/ 1 703 .03906,  20 1 7 .

[4]  Jianpeng  Cheng,  Li  Dong,  and  Mirella  Lapata.  Long  short-term  memory-networks  for  machine

reading.  arXiv p  reprint  arXiv: 1 601 . 06733,  20 1 6 .

1 0

[5]  Kyunghyun  Cho,  B art  van  Merrienboer,  Caglar  Gulcehre,  Fethi  Bougares,  Holger  Schwenk,

and  Yoshua  Bengio.  Learning  phrase  representations  using  rnn  encoder-decoder  for  statistical

machine  translation.  CoRR,  abs/ 1 406 . 1 07 8 ,  20 1 4.

[6]  Francois  Chollet.  Xception:  Deep  learning  with  depthwise  separable  convolutions .  arXiv

preprint  arXiv: 1 61 0. 02357,  20 1 6 .

[7]  Junyoung  Chung,  Çaglar  Gülçehre,  Kyunghyun  Cho,  and  Yoshua  Bengio.  Empirical  evaluation

of  gated  recurrent  neural  networks  on  sequence  modeling.  CoRR,  abs/ 1 4 1 2. 3555 ,  20 1 4.

[8]  Chris  Dyer,  Adhiguna  Kuncoro,  Miguel  B allesteros,  and  Noah  A.  Smith.  Recurrent  neural

network  grammars .  In  Proc.  ofN  AACL,  20 1 6.

[9]  Jonas  Gehring,  Michael  Auli,  David  Grangier,  Denis  Yarats,  and  Yann  N.  Dauphin.  Convolu

tional  sequence  to  sequence  learning.  arXiv p  reprint  arXiv: 1 705. 031 22v2 ,  20 1 7 .

[ 1 0]  Alex  Graves .  Generating  sequences  with  recurrent  neural  networks .  arXiv p  reprint

arXiv: 1 308. 0850,  20 1 3 .

[ 1 1 ]  Kaiming  He,  Xiangyu  Zhang,  Shaoqing  Ren,  and  Jian  Sun.  Deep  residual  learning  for  im

age  recognition.  In  Proceedings  of  the I  EEE  Conference  on  Computer  Vision  and  Pattern

Recognition,  pages  770–77 8 ,  20 1 6 .

[ 1 2]  Sepp  Hochreiter,  Yoshua  Bengio,  Paolo  Frasconi,  and  Jürgen  Schmidhuber.  Gradient  flow  in

recurrent  nets :  the  difficulty  of  learning  long-term  dependencies,  200 1 .

[ 1 3]  Sepp  Hochreiter  and  Jürgen  Schmidhuber.  Long  short-term  memory.  Neural  computation,

9 (8) : 1 7 3 5– 1 7 80,   1 997 .

[ 1 4]  Zhongqiang  Huang  and  Mary  Harper.  Self-training  PCFG  grammars  with  latent  annotations

across  languages .  In  Proceedings  of  the  2009  Conference  on  Empirical M  ethods  in N  atural

Language  Processing,  pages  832–84 1 .  ACL,  August  2009 .

[ 1 5]  Rafal  Jozefowicz,  Oriol  Vinyals,  Mike  Schuster,  Noam  Shazeer,  and  Yonghui  Wu.  Exploring

the  limits  of  language  modeling.  arXiv p  reprint  arXiv: 1 602. 0241 0,  20 1 6 .

[ 1 6]  Łukasz  Kaiser  and  Samy  Bengio.  Can  active  memory  replace  attention?  In  Advances  in N  eural

Information  Processing  Systems,  (NIPS),  20 1 6.

[ 1 7]  Łukasz  Kaiser  and  Ilya  Sutskever.  Neural  GPUs  learn  algorithms .  In  International  Conference

on L  earning R  epresentations  (ICLR),  20 1 6.

[ 1 8]  Nal  Kalchbrenner,  Lasse  Espeholt,  Karen  Simonyan,  Aaron  van  den  Oord,  Alex  Graves,  and  Ko

ray  Kavukcuoglu.  Neural  machine  translation  in  linear  time.  arXiv p  reprint  arXiv: 1 61 0. 1 0099v2 ,

20 1 7 .

[ 1 9]  Yoon  Kim,  Carl  Denton,  Luong  Hoang,  and  Alexander  M.  Rush.  Structured  attention  networks .

In  International  Conference  on L  earning R  epresentations,  20 1 7 .

[20]  Diederik  Kingma  and  Jimmy  B a.  Adam:  A  method  for  stochastic  optimization.  In  ICLR,  20 1 5 .

[2 1 ]  Oleksii  Kuchaiev  and  B oris  Ginsburg.  Factorization  tricks  for  LSTM  networks .  arXiv p  reprint

arXiv: 1 703. 1 0722 ,  20 1 7 .

[22]  Zhouhan  Lin,  Minwei  Feng,  Cicero  Nogueira  dos  Santos,  Mo  Yu,  Bing  Xiang,  Bowen

Zhou,  and  Yoshua  Bengio.  A  structured  self-attentive  sentence  embedding.  arXiv p  reprint

arXiv: 1 703. 031 30,  20 1 7 .

[23 ]  Minh-Thang  Luong,  Quoc  V.  Le,  Ilya  Sutskever,  Oriol  Vinyals,  and  Lukasz  Kaiser.  Multi-task

sequence  to  sequence  learning .  arXiv p  reprint  arXiv: 1 51 1 . 061 14,  20 1 5 .

[24]  Minh-Thang  Luong,  Hieu  Pham,  and  Christopher  D  Manning.  Effective  approaches  to  attention

based  neural  machine  translation.  arXiv p  reprint  arXiv: 1 508. 04025,  20 1 5 .

1 1

[25]  Mitchell  P  Marcus,  Mary  Ann  Marcinkiewicz,  and  Beatrice  S antorini.  Building  a  large  annotated

corpus  of  english:  The  penn  treebank.  Computational  linguistics,   1 9(2) : 3 1 3–3 30,   1 993 .

[26]  David  McClosky,  Eugene  Charniak,  and  Mark  Johnson.  Effective  self-training  for  parsing.  In

Proceedings  of the H  uman L  anguage  Technology  Conference  of the N  AACL, M  ain  Conference,

pages   1 52– 1 59 .  ACL,  June  2006 .

[27]  Ankur  Parikh,  Oscar  Täckström,  Dipanj an  Das,  and  Jakob  Uszkoreit.  A  decomposable  attention

model.  In  Empirical M  ethods  in N  atural L  anguage  Processing,  20 1 6.

[28]  Romain  Paulus,  Caiming  Xiong,  and  Richard  Socher.  A  deep  reinforced  model  for  abstractive

summarization.  arXiv p  reprint  arXiv: 1 705. 04304,  20 1 7 .

[29]  Slav  Petrov,  Leon  B arrett,  Romain  Thibaux,  and  Dan  Klein.  Learning  accurate,  compact,

and  interpretable  tree  annotation.  In  Proceedings  of  the  21 st I  nternational  Conference  on

Computational L  inguistics  and  44th A  nnual M  eeting  of  the A  CL,  pages  43 3–440.  ACL,  July

2006 .

[30]  Ofir  Press  and  Lior  Wolf.  Using  the  output  embedding  to  improve  language  models .  arXiv

preprint  arXiv: 1 608. 05859,  20 1 6.

[3 1 ]  Rico  Sennrich,  B arry  Haddow,  and  Alexandra  Birch.  Neural  machine  translation  of  rare  words

with  subword  units .  arXiv p  reprint  arXiv: 1 508. 07909,  20 1 5 .

[32]  Noam  Shazeer,  Azalia  Mirhoseini,  Krzysztof  Maziarz,  Andy  Davis,  Quoc  Le,  Geoffrey  Hinton,

and  Jeff  Dean.  Outrageously  large  neural  networks :  The  sparsely-gated  mixture-of-experts

layer.  arXiv p  reprint  arXiv: 1 701 . 06538,  20 1 7 .

[3 3 ]  Nitish  Srivastava,  Geoffrey  E  Hinton,  Alex  Krizhevsky,  Ilya  Sutskever,  and  Ruslan  S alakhutdi

nov.  Dropout:  a  simple  way  to  prevent  neural  networks  from  overfitting.  Journal  ofM  achine

Learning R  esearch,   1 5 ( 1 ) : 1 929– 1 95 8 ,  20 1 4 .

[34]  Sainbayar  Sukhbaatar,  Arthur  Szlam,  Jason  Weston,  and  Rob  Fergus .  End-to-end  memory

networks .  In  C .  Cortes ,  N.  D .  Lawrence,  D .  D .  Lee,  M .  Sugiyama,  and  R.  Garnett,  editors ,

Advances  in N  eural I  nformation  Processing  Systems  28,  pages  2440–2448 .  Curran  Associates,

Inc . ,  20 1 5 .

[35]  Ilya  Sutskever,  Oriol  Vinyals,  and  Quoc  VV  Le.  Sequence  to  sequence  learning  with  neural

networks .  In  Advances  in N  eural I  nformation  Processing  Systems,  pages  3 1 04–3 1 1 2,  20 1 4.

[36]  Christian  Szegedy,  Vincent  Vanhoucke,  Sergey  Ioffe,  Jonathon  Shlens,  and  Zbigniew  Wojna.

Rethinking  the  inception  architecture  for  computer  vision.  CoRR,  abs/ 1 5 1 2.00567 ,  20 1 5 .

[37]  Vinyals  &  Kaiser,  Koo,  Petrov,  Sutskever,  and  Hinton.  Grammar  as  a  foreign  language.  In

Advances  in N  eural I  nformation  Processing  Systems,  20 1 5 .

[3 8]  Yonghui  Wu,  Mike  Schuster,  Zhifeng  Chen,  Quoc  V  Le,  Mohammad  Norouzi,  Wolfgang

’

Macherey,  Maxim  Krikun,  Yuan  Cao,  Qin  Gao,  Klaus  Macherey,  et  al.  Google s  neural  machine

translation  system:  Bridging  the  gap  between  human  and  machine  translation.  arXiv p  reprint

arXiv: 1 609. 08144,  20 1 6 .

[39]  Jie  Zhou,  Ying  Cao,  Xuguang  Wang,  Peng  Li,  and  Wei  Xu.  Deep  recurrent  models  with

fast-forward  connections  for  neural  machine  translation.  CoRR,  abs/ 1 606 .04 1 99,  20 1 6 .

[40]  Muhua  Zhu,  Yue  Zhang,  Wenliang  Chen,  Min  Zhang,  and  Jingbo  Zhu.  Fast  and  accurate

shift-reduce  constituent  parsing.  In  Proceedings  of the  51 st A  nnual M  eeting  of the A  CL  (Volume

1 : L  ong  Papers),  pages  434–443 .  ACL,  August  20 1 3 .

1 2

I n ut I n ut La er 5

-

p p y

# Attention  Visualizations

s

t

n

n

e

o

n

i

t

a m

y
 s

a t

t c d
 g
 l >

i n r s

> > > > > >

i

r r

r e n t g
 u

S

i e

t  e

9 s e
 d d d d d d

i o e e
 s 
 n

e s
 i c c

r j c O

t
 k i r i

i v v s w
 0 a a a a a a

s  t f

i a a a e g o o

w n 
 f

m

o a a e 0 r r i E p p p p p p

p f
 i 
 o

h h h e

t s n a

I i i t s t a
 m o A g h p n l s 2 m t r o v p m d .
 < < < < < < <

t
 t
 t
 f
 r t

.

I s
 n s
 i a y
 n s
 e d s
 e 9 g e n g s
 e l > > > > > > >

i r t t w

a o o r

i i

i i c s u

a v e 0 n h o n d d d d d d

h r n e w i i i S

h t

o

t p c a s n t t e c

t 0 a a a a a a

i n a i k i

o e

l O

s r h s c f

j 2 a o m p p p p p p

s a f

a m a v o E

r i

e

t < < < < < <

r

p m

d

n s <

m i p

m

r

g

A

e

e

v

r

o

g

## Fi ure  3 :  An  exam le  of  the  attention  mechanism  followin  lon distance  de endencies  in  the

## -

## g p g g p

## encoder  self attention  in  la er  5  of  6 .  Man  of  the  attention  heads  attend  to  a  distant  de endenc  of

## -

## y y p y

## ‘ ’ ‘ ’

## the  verb   makin  com letin  the   hrase   makin . . .more  difficult .  Attentions  here  shown  onl  for

## g p g p g y

## ,

## ‘ ’

## the  word   makin .  Different  colors  re resent  different  heads .  B est  viewed  in  color.

## g p

## 1 3

I n ut I n ut La er 5

-

p p y

n

o

i

t

g

t
 n >

a

d

c n >

c i o

r l

S

i i

e l d

t

e u

s

t
 a O

n

f

w  v t
 o s s

e  l  r p i a

i e
 i

s y

l

i h e

h u p r p E p

a e e e h e

s

h

t u s n

T L n b p , b i a s b - t i a m , i m o . < <

w 
 
 w w

j

r t
 
 t
 t
 - t

l
 , , .

e e s
 n d e s
 s
 e e g n y
 n > >

l

w
 c u t l s i

i

e a r

i i

h b o b n o

d

i

e i u u w i m i

a h S

w v a

b h

I n ut I n ut La ea r 5 w

t

T - a

f t j

L s n

O

e r o

p p y

p

i

s

h

n e i

c p E

<

s

p

i

o <

m

l

p

p

a

n

o

i

t

g

t
 >

n

a

d

c n >

c i o

r l

S

i i

e l d

t

e u

s

t
 a O

n

f

w  v t
 o s s

e  l  r p i a

i e
 i

s y

l

i h e

h u p r p E p

a e e e h e

s

h

t u s n

T L n b p , b i a s b - t i a m , i m o . < <

w 
 
 w w

j

r t
 
 t
 t
 - t

l
 , , .

e e s
 n d e s
 s
 e e g n y
 n > >

l

w
 c u t l s i

i

e a r

i i

h b o b n o

d

i

e i u u w i m i

a h S

w v a

b h

t

T a

f t j

L s

r o n

O

e

a w

p

i

s

h

n e i

c p E

<

s

p

i

o <

m

l

p

p

a

# Fi ure  4 :  Two  attention  heads  also  in  la er  5  of  6  a arentl  involved  in  ana hora  resolution.  To :

# g y pp y p p

# , ,

# ‘ ’

# Full  attentions  for  head  5 .  B ottom:  Isolated  attentions  from   ust  the  word   its  for  attention  heads  5

# j

# and  6 .  Note  that  the  attentions  are  ver  shar  for  this  word.

# y p

# 1 4

I n ut I n ut La er 5

-

p p y

n

o

i

t

g

t
 n >

a

d

c n >

c i o

r l

S

i i

e l d

t

e u

s

t
 a O

n

f

w  v t
 o s s

e  l  r p i a

i e
 i

s y

l

i h e

h u p r p E p

a e e e h e

s

h

t u s n

T L n b p , b i a s b - t i a m , i m o . < <

w 
 
 w w

j

r t
 
 t
 t
 - t

l
 , , .

e e s
 n d e s
 s
 e e g n y
 n > >

l

w
 c u t l s i

i

e a r

i i

h b o b n o

d

i

e i u u w i m i

a h S

w v a

b h

t

T a

f t j

I n ut I n ut La er 5 s

L s n

O

- e r o

a w

p p y

p

i

h

n e i

c p E

<

s

p

i

o <

m

l

p

p

a

n

o

i

t

g

t
 n >

a

d

c n >

c i o

r l

S

i i

e l d

t

e u

s

t
 a O

n

f

w  v t
 o s s

e  l  r p i a

i e
 i

s y

l

i h e

h u p r p E p

a e e e h e

s

h

t u s n

T L n b p , b i a s b - t i a m , i m o . < <

w 
 
 w w

j

r t
 
 t
 t
 - t

l
 , , .

e e s
 n d e s
 s
 e e g n y
 n > >

l

w
 c u t l s i

i

e a r

i i

h b o b n o

d

i

e i u u w i m i

a h S

w v a

b h

t

T a

f t j

L s n

O

e r o

a w

p

i

s

h

n e i

c p E

<

s

p

i

o <

m

l

p

p

a

# Fi ure  5 :  Man  of  the  attention  heads  exhibit  behaviour  that  seems  related  to  the  structure  of  the

# g y

# sentence.  We   ive  two  such  exam les  above  from  two  different  heads  from  the  encoder  self attention

# -

# g p

# ,

# at  la er  5  of  6 .  The  heads  clearl  learned  to   erform  different  tasks .

# y y p

# 1 5
