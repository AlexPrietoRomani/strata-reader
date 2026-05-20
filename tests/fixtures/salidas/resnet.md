## Deep  Residual  Learning  for  Image  Recognition

### Kaiming  He  Xiangyu  Zhang  Shaoqing  Ren  Jian  Sun

### Microsoft  Research

{kahe,  v-xiangz,  v-shren, j  iansun} @ microsoft.com

# 5

1 20
 20

0 Abstract
 )

2 (%r  )%
 56-layer

Deeper  neural  networks  are  more  difficult  to  train.  We
 orr (ro  20-la er

c e  10
 rr 10
 y

present  a  residual  learning f  ramework  to  ease  the  training
 ign 56-layer
 te

e in tes

of  networks  that  are  substantially  deeper  than  those  used
 ar

D  t 20-layer

previously.  We  explicitly  reformulate  the  layers  as  learn 0 
 0

0  1  2  3  4  5  6  0  1  2  3  4  5  6

0 ing  residual f  unctions  with  reference  to  the  layer  inputs,  in iter. ( 1 e4)
 iter. ( 1 e4)

1 stead  of  learning  unreferenced f  unctions.  We p  rovide  com Figure   1 .  Training  error  (left)  and  test  error  (right)  on  CIFAR- 1 0

]  prehensive  empirical  evidence  showing  that  these  residual
 with  20-layer  and  56-layer  “plain”  networks .  The  deeper  network

networks  are  easier  to  o timize,  and  can   ain  accurac   rom
 has  higher  training  error,  and  thus  test  error.  Similar  phenomena

###### V p g yf

on  ImageNet  is  presented  in  Fig .  4 .

considerably  increased  depth.  On  the I  mageNet  dataset  we

# C

. evaluate  residual  nets  with  a  depth  of up  to  1 52  layers—8 ×

s deeper  than  VGG  nets  [41]  but  still  having  lower  complex greatly  benefited  from  very  deep  models .

# c

[ ity. A  n  ensemble  of these  residual  nets  achieves  3. 57%  error
 Driven  by  the  significance  of  depth,  a  question  arises :  Is

on  the I  mageNet  test  set.  This  result  won  the  1 st p  lace  on  the
 learning  better  networks  as  easy  as  stacking  more  layers ?

# 1

ILSVRC  201 5  classification  task.  We  also p  resent  analysis
 An  obstacle  to  answering  this  question  was  the  notorious

# v

5 on  CIFAR-1 0  with  1 00  and  1 000  layers. 
 problem  of  vanishing/exploding  gradients  [ 1 ,  9] ,  which

8 The  depth  of  representations  is  of  central  importance
 hamper  convergence  from  the  beginning.  This  problem,

3 for  many  visual  recognition  tasks.  Solely  due  to  our  ex however,  has  been  largely  addressed  by  normalized  initial

3 tremely  deep  representations,  we  obtain  a  28%  relative  im ization  [23 ,  9,  37 ,  1 3 ]  and  intermediate  normalization  layers

0. provement  on  the  COCO  object  detection  dataset. D  eep
 [ 1 6] ,  which  enable  networks  with  tens  of  layers  to  start  con

2 residual  nets  are f  oundations  of our  submissions  to I  LSVRC
 verging  for  stochastic  gradient  descent  (SGD)  with  back

1 &  COCO  201 5  competitions 1
,  where  we  also  won  the  1 st
 propagation  [22] .

# 5

1 places  on  the  tasks  of I  mageNet  detection, I  mageNet  local When  deeper  networks  are  able  to  start  converging,  a

# :

ization,  COCO  detection,  and  COCO  segmentation. 
 degradation  problem  has  been  exposed:  with  the  network

# v

i depth  increasing,  accuracy  gets  saturated  (which  might  be

X 1 .  Introduction
 unsurprising)  and  then  degrades  rapidly.  Unexpectedly,

r such  degradation  is  not  caused  by  overfitting,  and  adding

# a

Deep  convolutional  neural  networks  [22,  2 1 ]  have  led
 more  layers  to  a  suitably  deep  model  leads  to  higher  train

to  a  series  of  breakthroughs  for  image  classification  [2 1 ,
 ing  error,  as  reported  in  [ 1 1 ,  42]  and  thoroughly  verified  by

50,  40] .  Deep  networks  naturally  integrate  low/mid/high our  experiments .  Fig .  1   shows  a  typical  example.

level  features  [50]  and  classifiers  in  an  end-to-end  multi The  de radation  (of  trainin  accurac )  indicates  that  not

“ ” g g y

layer  fashion,  and  the   levels  of  features  can  be  enriched
 all  systems  are  similarly  easy  to  optimize.  Let  us  consider  a

by  the  number  of  stacked  layers  (depth) .  Recent  evidence
 shallower  architecture  and  its  deeper  counterpart  that  adds

[4 1 ,  44]  reveals  that  network  depth  is  of  crucial  importance,
 more  layers  onto  it.  There  exists  a  solution  by  construction

and  the  leading  results  [4 1 ,  44,  1 3 ,  1 6]  on  the  challenging
 to  the  deeper  model :  the  added  layers  are  identity  mapping,

“ ”

ImageNet  dataset  [3 6]  all  exploit   very  deep  [4 1 ]  models ,
 and  the  other  layers  are  copied  from  the  learned  shallower

with  a  depth  of  sixteen  [4 1 ]  to  thirty  [ 1 6] .  Many  other  non model.  The  existence  of  this  constructed  solution  indicates

trivial  visual  recognition  tasks  [ 8 ,  1 2,  7 ,  3 2,  27]  have  also
 that  a  dee er  model  should   roduce  no  hi her  trainin  error

p p g g

1 ht tp : / / image - net . o rg / cha l l e nge s / L SVRC / 2 0 1 5 /  and
 than  its  shallower  counterpart.  But  experiments  show  that

ht tp : / /ms c o c o . o rg / dat a s et / # det e ct i o n s - cha l l e nge 2 0 1 5 .
 our  current  solvers  on  hand  are  unable  to  find  solutions  that

1

x
 ImageNet  test  set,  and  won  the  1 st p  lace  in  the I  LSVRC

2015  classification  competition.  The  extremely  deep  rep

we i g ht  l a ye 
r

resentations  also  have  excellent  generalization  performance

F( x ) re l 
u

x
 on  other  recognition  tasks ,  and  lead  us  to  further  win  the

weight laye
r i d e ntity
 1 st p  laces  on: I  mageNet  detection, I  mageNet  localization,

COCO  detection,  and  COCO  segmentation  in  ILSVRC  &

F(x ) +  x

re l  u COCO  20 1 5  competitions .  This  strong  evidence  shows  that

Figure  2.  Residual  learning :  a  building  block.
 the  residual  learning  principle  is  generic,  and  we  expect  that

it  is  applicable  in  other  vision  and  non-vision  problems .

are  comparably  good  or  better  than  the  constructed  solution

(or  unable  to  do  so  in  feasible  time) .
 2.  Related  Work

In  this  paper,  we  address  the  degradation  problem  by

introducing  a  deep  residual  learning  framework.  In Residual  Representations.  In  image  recognition,  VLAD

stead  of  hoping  each  few  stacked  layers  directly  fit  a
 [ 1 8]  is  a  representation  that  encodes  by  the  residual  vectors

desired  underlying  mapping,  we  explicitly  let  these  lay with  respect  to  a  dictionary,  and  Fisher  Vector  [30]  can  be

ers  fit  a  residual  mapping .  Formally,  denoting  the  desired
 formulated  as  a  probabilistic  version  [ 1 8]  of  VLAD .  B oth

underlying  mapping  as  H (x) ,  we  let  the  stacked  nonlinear
 of  them  are  powerful  shallow  representations  for  image  re

layers  fit  another  mapping  of  F(x)  : =  H (x) − x.  The  orig trieval  and  classification  [4,  48] .  For  vector  quantization,

inal  mapping  is  recast  into  F(x) + x.  We  hypothesize  that  it
 encoding  residual  vectors  [ 1 7]  is  shown  to  be  more  effec

is  easier  to  optimize  the  residual  mapping  than  to  optimize
 tive  than  encoding  original  vectors .

the  original,  unreferenced  mapping .  To  the  extreme,  if  an
 In  low-level  vision  and  computer  graphics,  for  solv

identity  mapping  were  optimal,  it  would  be  easier  to  push
 ing  Partial  Differential  Equations  (PDEs) ,  the  widely  used

the  residual  to  zero  than  to  fit  an  identity  mapping  by  a  stack
 Multigrid  method  [3 ]  reformulates  the  system  as  subprob

of  nonlinear  layers .
 lems  at  multiple  scales,  where  each  subproblem  is  respon

The  formulation  of  F x  + x  can  be  realized  b  feedfor sible  for  the  residual  solution  between  a  coarser  and  a  finer

( ) y

ward  neural  networks  with  “shortcut  connections”  (Fig .  2) .
 scale.  An  alternative  to  Multigrid  is  hierarchical  basis  pre

Shortcut  connections  [2,  34,  49]  are  those  skipping  one  or
 conditioning  [45 ,  46] ,  which  relies  on  variables  that  repre

more  layers .  In  our  case,  the  shortcut  connections  simply
 sent  residual  vectors  between  two  scales .  It  has  been  shown

perform  identity  mapping,  and  their  outputs  are  added  to
 [3 ,  45 ,  46]  that  these  solvers  converge  much  faster  than  stan

the  outputs  of  the  stacked  layers  (Fig .  2) .  Identity  short dard  solvers  that  are  unaware  of  the  residual  nature  of  the

cut  connections  add  neither  extra  parameter  nor  computa solutions .  These  methods  suggest  that  a  good  reformulation

tional  complexity.  The  entire  network  can  still  be  trained
 or  preconditioning  can  simplify  the  optimization.

end-to-end  by  SGD  with  backpropagation,  and  can  be  eas Shortcut  Connections.  Practices  and  theories  that  lead  to

ily  implemented  using  common  libraries  (e. g . ,  Caffe  [ 1 9] )
 shortcut  connections  [2,  34,  49]  have  been  studied  for  a  long

without  modifying  the  solvers .
 time.  An  early  practice  of  training  multi-layer  perceptrons

We  present  comprehensive  experiments  on  ImageNet
 (MLPs)  is  to  add  a  linear  layer  connected  from  the  network

[3 6]  to  show  the  degradation  problem  and  evaluate  our
 input  to  the  output  [34,  49] .  In  [44,  24] ,  a  few  interme

method.  We  show  that:   1 )  Our  extremely  deep  residual  nets
 diate  layers  are  directly  connected  to  auxiliary  classifiers

“ ”

are  easy  to  optimize,  but  the  counterpart   plain  nets  (that
 for  addressing  vanishing/exploding  gradients .  The  papers

simply  stack  layers)  exhibit  higher  training  error  when  the
 of  [3 9 ,  3 8 ,  3 1 ,  47]  propose  methods  for  centering  layer  re

depth  increases ;  2)  Our  deep  residual  nets  can  easily  enj oy
 sponses ,  gradients ,  and  propagated  errors ,  implemented  by

accuracy  gains  from  greatly  increased  depth,  producing  re shortcut  connections .  In  [44] ,  an  “inception”  layer  is  com

sults  substantially  better  than  previous  networks .
 posed  of  a  shortcut  branch  and  a  few  deeper  branches .

Similar  phenomena  are  also  shown  on  the  CIFAR- 1 0  set
 Concurrent  with  our  work,  “highway  networks”  [42,  43]

[20] ,  suggesting  that  the  optimization  difficulties  and  the
 present  shortcut  connections  with  gating  functions  [ 1 5 ] .

effects  of  our  method  are  not j  ust  akin  to  a  particular  dataset.
 These  gates  are  data-dependent  and  have  parameters ,  in

We  present  successfully  trained  models  on  this  dataset  with
 contrast  to  our  identity  shortcuts  that  are  parameter-free.

over   1 00  layers ,  and  explore  models  with  over   1 000  layers .
 When  a  gated  shortcut  is  “closed”  (approaching  zero) ,  the

On  the  ImageNet  classification  dataset  [3 6] ,  we  obtain
 layers  in  highway  networks  represent  non-residual  func

excellent  results  by  extremely  deep  residual  nets .  Our   1 52-
 tions .  On  the  contrary,  our  formulation  always  learns

layer  residual  net  is  the  deepest  network  ever  presented  on
 residual  functions ;  our  identity  shortcuts  are  never  closed,

ImageNet,  while  still  having  lower  complexity  than  VGG
 and  all  information  is  always  passed  through,  with  addi

nets  [4 1 ] .  Our  ensemble  has  3.57 %   top-5  error  on  the
 tional  residual  functions  to  be  learned.  In  addition,  high

2

way  networks  have  not  demonstrated  accuracy  gains  with
 ReLU  [29]  and  the  biases  are  omitted  for  simplifying  no

extremely  increased  depth  (e. g . ,  over   1 00  layers) .
 tations .  The  operation  F  +  x  is  performed  by  a  shortcut

connection  and  element-wise  addition.  We  adopt  the  sec

3.  Deep  Residual  Learning
 ond  nonlinearity  after  the  addition  (i. e . ,  σ (y ) ,  see  Fig .  2) .

The  shortcut  connections  in  Eqn. ( 1 )  introduce  neither  ex

3. 1.  Residual  Learning

tra  parameter  nor  computation  complexity.  This  is  not  only

Let  us  consider  H (x)   as  an  underlying  mapping  to  be
 attractive  in  practice  but  also  important  in  our  comparisons

fit  by  a  few  stacked  layers  (not  necessarily  the  entire  net) ,
 between  plain  and  residual  networks .  We  can  fairly  com

with  x  denoting  the  inputs  to  the  first  of  these  layers .  If  one
 pare  plain/residual  networks  that  simultaneously  have  the

hypothesizes  that  multiple  nonlinear  layers  can  asymptoti same  number  of  parameters,  depth,  width,  and  computa

cally  approximate  complicated  functions2
 ,  then  it  is  equiv tional  cost  (except  for  the  negligible  element-wise  addition) .

alent  to  hypothesize  that  they  can  asymptotically  approxi The  dimensions  of  x  and  F  must  be  equal  in  Eqn. ( 1 ) .

mate  the  residual  functions ,  i. e . ,  H (x)   −  x  (as suming  that
 If  this  is  not  the  case  (e. g . ,  when  changing  the  input/output

the  input  and  output  are  of  the  same  dimensions) .  So
 channels) ,  we  can  perform  a  linear  proj ection  Ws  by  the

rather  than  expect  stacked  layers  to  approximate  H (x) ,  we
 shortcut  connections  to  match  the  dimensions :

explicitly  let  these  layers  approximate  a  residual  function

F(x)  : =  H (x)   −  x.  The  original  function  thus  becomes
 y  =  F(x ,  { Wi } )  +  Ws x .  (2)

F(x) + x.  Although  both  forms  should  be  able  to  asymptot

icall  a roximate  the  desired  functions  (as  h othesized) ,
 We  can  also  use  a  square  matrix  Ws  in  Eqn. ( 1 ) .  But  we  will

y pp yp

the  ease  of  learnin  mi ht  be  different.
 show  by  experiments  that  the  identity  mapping  is  sufficient

g g

This  reformulation  is  motivated  b  the  counterintuitive
 for  addressing  the  degradation  problem  and  is  economical,

y

henomena  about  the  de radation   roblem  (Fi .  1 ,  left) .  As
 and  thus  Ws  is  only  used  when  matching  dimensions .

p g p g 

we  discussed  in  the  introduction,  if  the  added  layers  can
 The  form  of  the  residual  function  F  is  flexible.  Exper

be  constructed  as  identit  ma in s ,  a  dee er  model  should
 iments  in  this  paper  involve  a  function  F  that  has  two  or

y pp g p

have  trainin  error  no   reater  than  its  shallower  counter three  layers  (Fig .  5) ,  while  more  layers  are  pos sible.  But  if

g g

art.  The  de radation   roblem  su ests  that  the  solvers
 F  has  only  a  single  layer,  Eqn. ( 1 )  is  similar  to  a  linear  layer:

p g p gg

mi ht  have  difficulties  in  a roximatin  identit  ma in s
 y  =  W1 x  +  x,  for  which  we  have  not  observed  advantages .

g pp g y pp g

by  multiple  nonlinear  layers .  With  the  residual  learning  re We  also  note  that  although  the  above  notations  are  about

formulation,  if  identity  mappings  are  optimal,  the  solvers
 fully-connected  layers  for  simplicity,  they  are  applicable  to

may  simply  drive  the  weights  of  the  multiple  nonlinear  lay convolutional  layers .  The  function  F(x ,  { Wi } )  can  repre

ers  toward  zero  to  a roach  identit  ma in s .
 sent  multiple  convolutional  layers .  The  element-wise  addi

pp y pp g

In  real  cases ,  it  is  unlikel  that  identit  ma in s  are  o  tion  is  performed  on  two  feature  maps ,  channel  by  channel.

y y pp g p

timal,  but  our  reformulation  may  help  to  precondition  the

3.3.  Network  Architectures

problem.  If  the  optimal  function  is  closer  to  an  identity

mapping  than  to  a  zero  mapping,  it  should  be  easier  for  the
 We  have  tested  various  plain/residual  nets ,  and  have  ob

solver  to  find  the  perturbations  with  reference  to  an  identity
 served  consistent  phenomena.  To  provide  instances  for  dis

mapping,  than  to  learn  the  function  as  a  new  one.  We  show
 cussion,  we  describe  two  models  for  ImageNet  as  follows .

by  experiments  (Fig .  7)  that  the  learned  residual  functions  in

general  have  small  responses,  suggesting  that  identity  map

pings  provide  reasonable  preconditioning.

Plain  Network.  Our  plain  baselines  (Fig .  3 ,  middle)  are

mainly  inspired  by  the  philosophy  of  VGG  nets  [4 1 ]  (Fig .  3 ,

left) .  The  convolutional  layers  mostly  have  3 × 3  filters  and

3.2.  Identity  Mapping  by  Shortcuts
 follow  two  simple  design  rules :  (i)  for  the  same  output

feature  map  size,  the  layers  have  the  same  number  of  fil

We  adopt  residual  learning  to  every  few  stacked  layers .
 ters ;  and  (ii)  if  the  feature  map  size  is  halved,  the  num

A  building  block  is  shown  in  Fig .  2 .  Formally,  in  this  paper
 ber  of  filters  is  doubled  so  as  to  preserve  the  time  com

we  consider  a  building  block  defined  as :
 plexity  per  layer.  We  perform  downsampling  directly  by

convolutional  layers  that  have  a  stride  of  2 .  The  network

y  =  F(x ,  { Wi } )  +  x .  ( 1 )
 ends  with  a  global  average  pooling  layer  and  a   1 000-way

 fully-connected  layer  with  softmax.  The  total  number  of

Here  x  and  y  are  the  input  and  output  vectors  of  the  lay

weighted  layers  is  34  in  Fig .  3  (middle) .

ers  considered.  The  function  F(x ,  { Wi } )   represents  the

It  is  worth  noticing  that  our  model  has  fewer  filters  and

residual  mapping  to  be  learned.  For  the  example  in  Fig .  2

= lower  complexity  than  VGG  nets  [4 1 ]  (Fig .  3 ,  left) .  Our  34-

that  has  two  layers,  F    W2 σ ( W1 x)   in  which  σ  denotes

layer  baseline  has  3 . 6  billion  FLOPs  (multiply-adds) ,  which

2This  hypothesis,  however,  is  still  an  open  question.  See  [28] .
 is  only   1 8 %  of  VGG- 1 9  ( 1 9 . 6  billion  FLOPs) .

3

### VG G-19  34-l aye r p l a i 
n 34-l aye r res id u a
l Residual  Network.  B ased  on  the  above  plain  network,  we

## image
 image
 image
 insert  shortcut  connections  (Fig .  3 ,  right)  which  turn  the

## output 
 3x3 conv, 64
 network  into  its  counterpart  residual  version .  The  identity

s i z e :  2 24

###### 3x3 conv, 64
 shortcuts  (Eqn . ( 1 ))  can  be  directly  used  when  the  input  and

###### pool, /2
 output  are  of  the  same  dimensions  (solid  line  shortcuts  in

o u t p u t

## size: 112
 3x3 conv, 128
 Fig .  3 ) .  When  the  dimensions  increase  (dotted  line  shortcuts

# in  Fig .  3 ) ,  we  consider  two  options :  (A)  The  shortcut  still

3x3 co nv,  1 28
 7x7 co nv, 64, /2
 7x7 co nv, 64, /2

# performs  identity  mapping,  with  extra  zero  entries  padded

o u t p u t 
 p o o l , /2
 p o o l , /2
 p o o l , / 2

###### size: 56
 for  increasing  dimensions .  This  option  introduces  no  extra

3x3 co nv, 2 5 6
 3 x3 co nv, 64
 3 x3 co nv, 64

# parameter;  (B )  The  proj ection  shortcut  in  Eqn. (2)  is  used  to

3x3 co nv, 2 5 6
 3 x3 co nv, 64
 3 x3 co nv, 64

# match  dimensions  (done  by   1 × 1  convolutions) .  For  both

3x3 co nv, 2 5 6
 3 x3 co nv, 64
 3 x3 co nv, 64

# options ,  when  the  shortcuts  go  across  feature  maps  of  two

### 3x3 conv, 256
 3x3 conv, 64
 3x3 conv, 64
 size s ,  they  are  performed  with  a  stride  of  2 .

3 x3 co nv, 64
 3 x3 co nv, 64

###### 3x3 conv, 64
 3x3 conv, 64
 3 .4.  Implementation

### output 
 pool, /2
 3x3 conv, 128, /2
 3x3 conv, 128, /2
 Our  implementation  for  ImageNet  follow s  the  practice

### size : 28
 3x3 conv, 5 12
 3x3 conv, 128
 3x3 conv, 128
 in   [2 1 ,  4 1 ] .  The  image  i s  re sized  with  its  shorter  side  ran

### 3x3 conv, 512
 3x3 conv, 128
 3x3 conv, 128
 domly  s ampled  in  [2 5 6 ,   48 0]   for  scale  augmentation   [4 1 ] .

### 3x3 conv, 512
 3x3 conv, 128
 3x3 conv, 128
 A  224 × 224  crop  is  randomly  s ampled  from  an  image  or  its

###### 3x3 conv, 512
 3x3 conv, 128
 3x3 conv, 128
 horizontal  flip ,  with  the  per-pixel  mean  subtracted   [2 1 ] .  The

## 3x3 conv, 128
 3x3 conv, 128
 standard  color  augmentation  in   [2 1 ]  is  used .  We  adopt  batch

## 3x3 conv, 128
 3x3 conv, 128
 normalization  (B N)   [ 1 6]  right  after  each  convolution  and

## 3x3 conv, 128
 3x3 conv, 128
 before  activation,  following   [ 1 6] .  We  initialize  the  weights

### output 
 pool, /2
 3x3 conv, 256, /2
 3x3 conv, 256, /2
 as  in   [ 1 3 ]  and  train  all  plain/re sidual  nets  from  scratch .  We

##### s i ze :  1 4
 -

# use  S GD  with  a  mini batch  size  of  25 6 .  The  learning  rate

3x3 co nv, 5 1 2
 3x3 co nv, 2 5 6
 3x3 co nv, 2 5 6

# starts  from  0 . 1  and  is  divided  by   1 0  when  the  error  plateaus ,

###### 3x3 co nv, 5 1 2
 3x3 co nv, 2 5 6
 3x3 co nv, 2 5 6
 4

# and  the  models  are  trained  for  up  to  60  ×   1 0 iterations .  We

3x3 co nv, 5 1 2
 3x3 co nv, 2 5 6
 3x3 co nv, 2 5 6

# use  a  weight  decay  of  0.000 1  and  a  momentum  of  0.9 .  We

3x3 co nv, 5 1 2
 3x3 co nv, 2 5 6
 3x3 co nv, 2 5 6

# do  not  use  dropout   [ 1 4] ,  following  the  practice  in   [ 1 6] .

3x3 co nv, 2 5 6
 3x3 co nv, 2 5 6

# In  testing,  for  comparison  studies  we  adopt  the  standard

##### 3x3 co nv, 2 5 6
 3x3 co nv, 2 5 6
 - 

# 1 0 crop  testing  [2 1 ] .  For  best  results ,  we  adopt  the  fully

## 3x3 conv, 256
 3x3 conv, 256
 convolutional  form  as  in   [4 1 ,  1 3 ] ,  and  average  the  scores

## 3x3 conv, 256
 3x3 conv, 256
 at  multiple  scales  (images  are  resized  such  that  the  shorter

### 3x3 conv, 256
 3x3 conv, 256
 side  i s  in  { 2 2 4 ,   2 5 6 ,   3 8 4 ,   4 8 0 ,   6 4 0 } ) .

3x3 co nv, 2 5 6
 3x3 co nv, 2 5 6

#### 3x3 conv, 256
 3x3 conv, 256
 4 .  Experiments

os iuztep:  u7
 t 
 p o o l , /2
 3 x3 co n v, 5 1 2 , /2
 3x 3 co n v, 5 1 2 , /2

3x3 co nv, 5 1 2
 3x3 co nv, 5 1 2

###### 4.1.  ImageNet  Classification

## 3x3 conv, 512
 3x3 conv, 512
 We  evaluate  our  method  on  the  ImageNet  20 1 2  clas sifi

## 3x3 conv, 512
 3x3 conv, 512
 cation  dataset   [ 3 6]  that  consi sts  of   1 000  clas ses .  The  models

## 3x3 conv, 512
 3x3 conv, 512
 are  trained  on  the   1 . 2 8  million  training  images ,  and  evalu

## 3x3 conv, 512
 3x3 conv, 512
 ated  on  the  5 0k  validation  images .  We  also  obtain  a  final

###### output 
 fc 4096
 avg poo
l avg poo
l re sult  on  the   1 00k  te st  image s ,  reported  by  the  te st  server.

###### s i z e :   1
 - -

# We  evaluate  both  top 1  and  top 5  error  rates .

fc 40 9 6
 fc 100 0
 fc 1000

###### fc 1000
 Plain  Networks.  We  first  evaluate   1 8 -layer  and  34-layer

# plain  nets .  The  34-layer  plain  net  is  in  Fig .  3  (middle) .  The

# Figure  3 .  Example  network  architectures  for  ImageNet.  Left:  the
 1 8 -layer  plain  net  is  of  a  similar  form.  S ee  Table  1   for  de

###### VGG- 1 9  model  [4 1 ]  ( 1 9 . 6  billion  FLOPs)  as  a  reference.  Mid tailed  architectures .

# dle :  a  plain  network  with  34  parameter  layers  (3 . 6  billion  FLOPs) .
 The  results  in  Table  2  show  that  the  deeper  34-layer  plain

# Right:  a  residual  network  with  34  parameter  layers  (3 . 6  billion
 net  has  higher  validation  error  than  the  shallower   1 8 -layer

# FLOPs) .  The  dotted  shortcuts  increase  dimensions .  Table  1  shows
 plain  net.  To  reveal  the  reasons ,  in  Fig .  4  (left)  we  com

###### more  details  and  other  variants .
 

# pare  their  training/validation  errors  during  the  training  pro

# cedure.  We  have  observed  the  degradation  problem  -  the

# 4

layer  name   output  size   1 8-layer   34-layer   50-layer   1 0 1 -layer   1 52-layer

conv 1   1 1 2 × 1 1 2   7 × 7 ,  64,  stride  2

3 ×
 3  max  p o ol , 
  stride  2

conv2   x   5 6 × 5 6 
 
 3 × 3 ,  64 
 
 
 3 × 3 ,  64 
 
 1 × 1 ,  64 
 1 × 1 ,  64 
 1 × 1 ,  64

× 2 
 × 3 
  
 3 × 3 ,  64 
  × 3 
  
 3 × 3 ,  64 
  × 3 
  
 3 × 3 ,  64 
  × 3

 3 × 3 ,  64     3 × 3 ,  64     1 × 1 ,  25 6 
   1 × 1 ,  25 6 
   1 × 1 ,  25 6 
 

1 × 1 ,   1 2 8 
 
 
 1 × 1 ,   1 2 8 
 
 
 1 × 1 ,   1 2 8

conv3   x   2 8 × 2 8   3 × 3 ,   1 2 8 
 × 2 
 3 × 3 ,   1 2 8 
 × 4 
  
 3 × 3 ,   1 2 8 
  × 4 
  
 3 × 3 ,   1 2 8 
  × 4 
  
 3 × 3 ,   1 2 8 
  × 8

 3 × 3 ,   1 2 8     3 × 3 ,   1 2 8     1 × 1 ,  5 1 2 
   1 × 1 ,  5 1 2 
   1 × 1 ,  5 1 2 
 

1 × 1 ,  2 5 6 
 
 
 1 × 1 ,  2 5 6 
 
 
 1 × 1 ,  2 5 6

conv4   x   1 4 × 1 4   3 × 3 ,  25 6 
 × 2 
 3 × 3 ,  25 6 
 × 6 
  
 3 × 3 ,  25 6 
  × 6 
  
 3 × 3 ,  25 6 
  × 23 
  
 3 × 3 ,  25 6 
  × 3 6

 3 × 3 ,  25 6     3 × 3 ,  25 6     1 × 1 ,   1 024 
   1 × 1 ,   1 024 
   1 × 1 ,   1 024 
 

1 × 1 ,  5 1 2 
 
 
 1 × 1 ,  5 1 2 
 
 
 1 × 1 ,  5 1 2

conv5   x   7 × 7 
 3 × 3 ,  5 1 2 
 × 2 
 3 × 3 ,  5 1 2 
 × 3 
  
 3 × 3 ,  5 1 2 
  × 3 
  
 3 × 3 ,  5 1 2 
  × 3 
  
 3 × 3 ,  5 1 2 
  × 3

 3 × 3 ,  5 1 2     3 × 3 ,  5 1 2     1 × 1 ,  204 8 
   1 × 1 ,  204 8 
   1 × 1 ,  204 8 
 

1 × 1   average  pool,   1 000-d  fc,  softmax

FLOPs   1 . 8 × 1 09   3 . 6 × 1 09   3 . 8 × 1 09   7 . 6 × 1 09   1 1 . 3 × 1 09

Table   1 .  Architectures  for  ImageNet.  Building  blocks  are  shown  in  brackets  (see  also  Fig .  5) ,  with  the  numbers  of  blocks  stacked.  Down

sampling  is  performed  by  conv3  1 ,  conv4  1 ,  and  conv5  1  with  a  stride  of  2 .

6 0
 6 0

5 0
 5 0

)
 )

(%  (%

ror 40
 ror 40

er 3 4-l ayer
 er

1 8 -l ayer

3 0
 3 0

- 1 8 -l ayer
 -

plain 1 8
 ResNet 1 8

plain-3 4
 ResNet-3 4
 3 4-l ayer

2 0
 2 0

0  1 0  2 0  3 0  40  5 0  0  1 0  2 0  3 0  40  5 0

iter . ( 1 e4)
 iter . ( 1 e4)

Figure  4 .  Training  on  ImageNet.  Thin  curves  denote  training  error,  and  bold  curves  denote  validation  error  of  the  center  crops .  Left:  plain

networks  of   1 8  and  34  layers .  Right:  ResNets  of   1 8  and  34  layers .  In  this  plot,  the  residual  networks  have  no  extra  parameter  compared  to

their  plain  counterparts .

plain  ResNet
 reducing  of  the  training  error3
 .  The  reason  for  such  opti

1 8  layers  27 . 94  27 . 8 8
 mization  difficulties  will  be  studied  in  the  future .

34  layers  28 . 54  25.03

Residual  Networks.  Next  we  evaluate   1 8-layer  and  34-

Table  2.  Top- 1  error  (% ,   1 0-crop  testing)  on  ImageNet  validation.
 layer  residual  nets  (ResNets) .  The  baseline  architectures

Here  the  ResNets  have  no  extra  parameter  compared  to  their  plain
 are  the  same  as  the  above  plain  nets ,  expect  that  a  shortcut

counterparts .  Fig .  4  shows  the  training  procedures .

connection  is  added  to  each  pair  of  3 × 3  filters  as  in  Fig .  3

(right) .  In  the  first  comparison  (Table  2  and  Fig .  4  right) ,

we  use  identity  mapping  for  all  shortcuts  and  zero-padding

34-layer  plain  net  has  higher  training  error  throughout  the
 for  increasing  dimensions  (option  A) .  So  they  have  no  extra

whole  training  procedure,  even  though  the  solution  space
 parameter  compared  to  the  plain  counterparts .

of  the   1 8-layer  plain  network  is  a  subspace  of  that  of  the
 We  have  three  maj or  observations  from  Table  2  and

34-layer  one .
 Fig .  4 .  First,  the  situation  is  reversed  with  residual  learn

We  argue  that  this  optimization  difficulty  is  unlikely  to
 ing –   the  34-layer  ResNet  is  better  than  the   1 8-layer  ResNet

be  caused  by  vanishing  gradients .  These  plain  networks  are
 (by  2. 8 %) .  More  importantly,  the  34-layer  ResNet  exhibits

trained  with  BN  [ 1 6] ,  which  ensures  forward  propagated
 considerably  lower  training  error  and  is  generalizable  to  the

signals  to  have  non-zero  variances .  We  also  verify  that  the
 validation  data.  This  indicates  that  the  degradation  problem

backward  propagated  gradients  exhibit  healthy  norms  with
 is  well  addressed  in  this  setting  and  we  manage  to  obtain

BN.  So  neither  forward  nor  backward  signals  vanish.  In
 accuracy  gains  from  increased  depth.

fact,  the  34-layer  plain  net  is  still  able  to  achieve  compet Second,  compared  to  its  plain  counterpart,  the  34-layer

itive  accuracy  (Table  3 ) ,  suggesting  that  the  solver  works
 3 

We  have  experimented  with  more  training  iterations  (3 × )  and  still  ob

to  some  extent.  We  conj ecture  that  the  deep  plain  nets  may
 served  the  degradation  problem,  suggesting  that  this  problem  cannot  be

have  exponentially  low  convergence  rates ,  which  impact  the
 feasibly  addressed  by  simply  using  more  iterations.

5

- - 64-d  2 5 6-d

model  top 1  err.  top 5  err.

VGG- 1 6   [4 1 ]   2 8 . 07  9 . 3 3
 3x3, 64
 1x1, 64

r e l u

GoogLeNet   [44]   -  9 . 1 5
 re l u
 3x3, 64

PReLU-net   [ 1 3 ]   24 . 27  7 . 3 8
 3x3, 64
 rel u

1x 1, 2 5 6

plain-34  28 . 54   1 0 . 02

- r e l u
 r e l u

ResNet 34  A  25 . 03  7 .76

ResNet-34  B  24 . 5 2  7 .46

- Figure  5 .  A  deeper  residual  function  F  for  ImageNet.  Left:  a

ResNet 34  C  24 . 1 9  7 .40

building  block  (on  56 × 56  feature  maps)  as  in  Fig .  3  for  ResNet

ResNet-50  22 . 85  6 .7 1 
 “ ” -

34 .  Right:  a   bottleneck  building  block  for  ResNet 50/ 1 0 1 / 1 52.

ResNet- 1 0 1   2 1 . 75  6 . 05

ResNet- 1 52  21 .43  5.71

parameter-free,  identity  shortcuts  help  with  training .  Next

Table  3 .  Error  rates  (% ,  10-crop  testing)  on  ImageNet  validation.
 we  investi ate   ro ection  shortcuts  (E n. (2)) .  In  Table  3  we

VGG- 1 6  is  based  on  our  test.  ResNet-50/ 1 0 1 / 1 52  are  of  option  B
 g p j - q

compare  three  options :  (A)  zero padding  shortcuts  are  used

that  only  uses  proj ections  for  increasing  dimensions .
 

for  increasing  dimensions,  and  all  shortcuts  are  parameter

method  top- 1  err.  top-5  err.
 free  (the  same  as  Table  2  and  Fig .  4  right) ;  (B )  proj ec

VGG  [4 1 ]  (ILSVRC ’ 1 4)  -  8 .43 †
 tion  shortcuts  are  used  for  increasing  dimensions ,  and  other

Goo LeNet  [44]  (ILSVRC ’ 1 4)  -  7 . 89
 shortcuts  are  identity ;  and  (C)  all  shortcuts  are  proj ections .

g

VGG  [4 1 ]  (v5)  24.4  7 . 1 
 Table  3  shows  that  all  three  options  are  considerably  bet

PReLU-net  [ 1 3 ]  2 1 . 59  5 .7 1 
 ter  than  the  plain  counterpart.  B  is  slightly  better  than  A.  We

BN-ince tion  [ 1 6]  2 1 .99  5 . 8 1 
 argue  that  this  is  because  the  zero-padded  dimensions  in  A

p

ResNet-34  B  2 1 . 84  5 .7 1 
 indeed  have  no  residual  learning .  C  is  marginally  better  than

ResNet-34  C  2 1 .5 3  5 . 60
 B ,  and  we  attribute  this  to  the  extra  parameters  introduced

ResNet-50  20.74  5 .25
 by  many  (thirteen)  proj ection  shortcuts .  But  the  small  dif

ResNet- 1 0 1  1 9 . 87  4.60
 ferences  among  A/B/C  indicate  that  proj ection  shortcuts  are

- not  essential  for  addressing  the  degradation  problem.  So  we

ResNet 1 52  19.38  4.49

do  not  use  option  C  in  the  rest  of  this  paper,  to  reduce  mem

Table  4.  Error  rates  (%)  of  single-model  results  on  the  ImageNet
 ory/time  complexity  and  model  sizes .  Identity  shortcuts  are

validation  set  (except  †
 reported  on  the  test  set) .
 articularl  im ortant  for  not  increasin  the  com lexit  of

p y p g p y

method  top-5  err.  (test)

the  bottleneck  architectures  that  are  introduced  below.

VGG  [4 1 ]  (ILSVRC’ 1 4)  7 .32
 Deeper  Bottleneck  Architectures.  Next  we  describe  our

GoogLeNet  [44]  (ILSVRC ’ 1 4)  6.66
 deeper  nets  for  ImageNet.  B ecause  of  concerns  on  the  train

VGG  [4 1 ]  (v5)  6. 8
 ing  time  that  we  can  afford,  we  modify  the  building  block

PReLU-net  [ 1 3 ]  4.94
 as  a  bottleneck  design4
 .  For  each  residual  function  F,  we

BN-inception  [ 1 6]  4 . 82
 use  a  stack  of  3  layers  instead  of  2  (Fig .  5 ) .  The  three  layers

ResNet  (ILSVRC ’ 15)  3.57
 are   1 × 1 ,  3 × 3 ,  and   1 × 1  convolutions ,  where  the   1 × 1  layers

- are  responsible  for  reducing  and  then  increasing  (restoring)

Table  5 .  Error  rates  (%)  of  ensembles .  The  top 5  error  is  on  the

dimensions ,  leaving  the  3 × 3  layer  a  bottleneck  with  smaller

test  set  of  ImageNet  and  reported  by  the  test  server.

input/output  dimensions .  Fig.  5  shows  an  example,  where

both  designs  have  similar  time  complexity.

ResNet  reduces  the  top- 1  error  by  3 . 5 %  (Table  2) ,  resulting
 The  parameter-free  identity  shortcuts  are  particularly  im

from  the  successfully  reduced  training  error  (Fig .  4  right  vs.
 portant  for  the  bottleneck  architectures .  If  the  identity  short

left) .  This  comparison  verifies  the  effectivenes s  of  residual
 cut  in  Fig .  5  (right)  is  replaced  with  proj ection,  one  can

learning  on  extremely  deep  systems .
 show  that  the  time  complexity  and  model  size  are  doubled,

- as  the  shortcut  is  connected  to  the  two  high-dimensional

Last,  we  also  note  that  the   1 8 layer  plain/residual  nets

- ends .  So  identity  shortcuts  lead  to  more  efficient  models

are  comparably  accurate  (Table  2) ,  but  the   1 8 layer  ResNet

“ for  the  bottleneck  designs .

converges  faster  (Fig .  4  right  vs .  left) .  When  the  net  is   not

overly  deep”  ( 1 8  layers  here) ,  the  current  S GD  solver  is  still
 50-layer  ResNet:  We  replace  each  2-layer  block  in  the

able  to  find  good  solutions  to  the  plain  net.  In  this  case,  the
 4 -

Deeper  non bottleneck  ResNets  (e. g . ,  Fig .  5  left)  also  gain  accuracy

ResNet  eases  the  optimization  by  providing  faster  conver from  increased  depth  (as  shown  on  CIFAR- 1 0),  but  are  not  as  economical

gence  at  the  early  stage .
 as  the  bottleneck  ResNets .  So  the  usage  of  bottleneck  designs  is  mainly  due

to  practical  considerations .  We  further  note  that  the  degradation  problem

Identity  vs .  Proj ection  Shortcuts.  We  have  shown  that
 of  plain  nets  is  also  witnessed  for  the  bottleneck  designs .

6

34-layer  net  with  this  3 -layer  bottleneck  block,  resulting  in
 method  error  (%)

a  50-layer  ResNet  (Table  1 ) .  We  use  option  B  for  increasing
 Maxout  [ 1 0]  9 . 3 8

dimensions .  This  model  has  3 . 8  billion  FLOPs .
 NIN  [25]  8 . 8 1

101 -layer  and  152-layer  ResNets :  We  construct   1 0 1 -
 DSN  [24]  8 .22

layer  and   1 52-layer  ResNets  by  using  more  3 -layer  blocks
 #  layers  #  params

(Table  1 ) .  Remarkably,  although  the  depth  is  significantly
 FitNet  [3 5]  1 9  2. 5M  8 . 39

increased,  the   1 5 2-layer  ResNet  ( 1 1 . 3  billion  FLOPs)  still
 Highway  [42,  43 ]  1 9  2. 3M  7 . 54  (7 .72±0. 1 6)

has  lower  complexity  than  VGG- 1 6/ 1 9  nets  ( 1 5 . 3/ 1 9 . 6  bil Highway  [42,  43 ]  32  1 .25M  8 . 80

lion  FLOPs) .
 ResNet  20  0. 27M  8 .75

The  50/ 1 0 1 / 1 52-layer  ResNets  are  more  accurate  than
 ResNet  32  0.46M  7 .5 1

the  34-layer  ones  by  considerable  margins  (Table  3  and  4) .
 ResNet  44  0. 66M  7 . 1 7

We  do  not  observe  the  degradation  problem  and  thus  en ResNet  56  0. 85M  6.97

j oy  significant  accuracy  gains  from  considerably  increased
 ResNet  1 1 0  1 .7M  6.43  (6.6 1 ±0. 1 6)

depth.  The  benefits  of  depth  are  witnessed  for  all  evaluation
 ResNet  1 202  1 9 .4M  7 .93

metrics  (Table  3  and  4) .

Table  6 .  Classification  error  on  the  CIFAR- 10  test  set.  All  meth

Comparisons  with  State-of-the-art  Methods.  In  Table  4
 ods  are  with  data  augmentation.  For  ResNet- 1 1 0,  we  run  it  5  times

we  compare  with  the  previous  best  single-model  results .
 and  show  “best  (mean± std)”  as  in  [43] .

Our  baseline  34-layer  ResNets  have  achieved  very  compet

itive  accuracy.  Our   1 52-layer  ResNet  has  a  single-model

top-5  validation  error  of  4 .49 % .  This  single-model  result
 so  our  residual  models  have  exactly  the  same  depth,  width,

outperforms  all  previous  ensemble  results  (Table  5) .  We
 and  number  of  parameters  as  the  plain  counterparts .

combine  six  models  of  different  depth  to  form  an  ensemble
 We  use  a  weight  decay  of  0.000 1  and  momentum  of  0.9,

(only  with  two   1 5 2-layer  ones  at  the  time  of  submitting) .
 and  adopt  the  weight  initialization  in  [ 1 3 ]  and  BN  [ 1 6]  but

This  leads  to  3.57 %   top-5  error  on  the  test  set  (Table  5) .
 with  no  dropout.  These  models  are  trained  with  a  mini

This  entry  won  the  1 st p  lace  in I  LSVRC  201 5. 
 batch  size  of   1 28  on  two  GPUs .  We  start  with  a  learning

rate  of  0 . 1 ,  divide  it  by   1 0  at  3 2k  and  48k  iterations ,  and

4.2.  CIFAR- 10  and  Analysis
 terminate  training  at  64k  iterations,  which  is  determined  on

a  45k/5k  train/val  split.  We  follow  the  simple  data  augmen

We  conducted  more  studies  on  the  CIFAR- 1 0  dataset

tation  in   [24]  for  training :  4  pixels  are  padded  on  each  side,

[20] ,  which  consists  of  50k  training  images  and   1 0k  test

and  a  32 × 32  crop  is  randomly  sampled  from  the  padded

ing  images  in   1 0  classes .  We  present  experiments  trained

image  or  its  horizontal  flip .  For  testing,  we  only  evaluate

on  the  training  set  and  evaluated  on  the  test  set.  Our  focus

the  single  view  of  the  original  3 2 × 3 2  image.

is  on  the  behaviors  of  extremely  deep  networks,  but  not  on
 =

We  compare  n    { 3 ,  5 ,  7 ,  9 } ,  leading  to  20,  3 2,  44 ,  and

pushing  the  state-of-the-art  results ,  so  we  intentionally  use
 -

5 6 layer  networks .  Fig .  6  (left)  shows  the  behaviors  of  the

simple  architectures  as  follows .

plain  nets .  The  deep  plain  nets  suffer  from  increased  depth,

The  plain/residual  architectures  follow  the  form  in  Fig .  3

and  exhibit  higher  training  error  when  going  deeper.  This

(middle/right) .  The  network  inputs  are  32 × 32  images,  with

phenomenon  is  similar  to  that  on  ImageNet  (Fig .  4,  left)  and

the  per-pixel  mean  subtracted.  The  first  layer  is  3 × 3  convo

on  MNIST  (see  [42] ) ,  suggesting  that  such  an  optimization

lutions .  Then  we  use  a  stack  of  6n  layers  with  3 × 3  convo

difficulty  is  a  fundamental  problem.

lutions  on  the  feature  maps  of  sizes  { 3 2 ,  1 6 ,  8 }  respectively,
 Fi .  6  (middle)  shows  the  behaviors  of  ResNets .  Also

g

with  2n  layers  for  each  feature  map  size.  The  numbers  of

similar  to  the  ImageNet  cases  (Fig .  4,  right) ,  our  ResNets

filters  are  { 1 6 ,  32 ,  64 }  respectively.  The  subsampling  is  per mana e  to  overcome  the  o timization  difficult  and  demon

g p y

formed  by  convolutions  with  a  stride  of  2.  The  network  ends

strate  accuracy  gains  when  the  depth  increases .

with  a  global  average  pooling,  a   1 0-way  fully-connected
 We  further  ex lore  n  =  1 8  that  leads  to  a   1 1 0-la er

p y

layer,  and  softmax.  There  are  totally  6n+2  stacked  weighted

ResNet.  In  this  case,  we  find  that  the  initial  learning  rate

layers .  The  following  table  summarizes  the  architecture :
 5

of  0 . 1  is  slightly  too  large  to  start  converging .  S o  we  use

0 . 0 1  to  warm  up  the  training  until  the  training  error  is  below

output  map  size  3 2 × 3 2  1 6 × 1 6  8 × 8

80%  (about  400  iterations) ,  and  then  go  back  to  0. 1  and  con

#  layers  1 +2n  2n  2n

tinue  training .  The  rest  of  the  learning  schedule  is  as  done

#  filters  1 6  3 2  64
 -

previously.  This   1 1 0 layer  network  converges  well  (Fig .  6,

When  shortcut  connections  are  used,  they  are  connected
 middle) .  It  has  fewer  parameters  than  other  deep  and  thin

to  the  pairs  of  3 × 3  layers  (totally  3n  shortcuts) .  On  this
 5 With  an  initial  learning  rate  of  0. 1 ,  it  starts  converging  ( < 90%  error)

dataset  we  use  identity  shortcuts  in  all  cases  (i. e . ,  option  A) ,
 after  several  epochs,  but  still  reaches  similar  accuracy.

7

20
 20
 ResNet-20
 20
 residual- 1 1 0

ResNet-3 2
 residual- 1 202

ResNet-44

ResNet-5 6

5 6 -layer  ResNet- 1 1 0

)%
 )%
 )%

(ror  1 0
 20 -la er
 (ror  1 0
 20-layer
 (ror  1 0

er y er er

5
 plain-2 0
 5
 1 1 0 -lay er
 5

plain- 3 2

plain-44

0
 p lain- 5 6
 0
 0

1

0   1 2  3  4  5  6  0   1 2  3  4  5  6  4  5  6

iter . ( 1 e4)
 iter . ( 1 e4)
 iter .  ( 1 e4)

Figure  6 .  Training  on  CIFAR- 10.  Dashed  lines  denote  training  error,  and  bold  lines  denote  testing  error.  Left:  plain  networks .  The  error

of  plain- 1 1 0  is  higher  than  60%  and  not  displayed.  Middle :  ResNets .  Right:  ResNets  with   1 1 0  and   1 202  layers .

3
 plain--20
 training  data  07 + 1 2  07 ++ 1 2

plain 5 6

dt
 2
 RReessNNeett--2506

 test  data  VOC  07  test  VOC   1 2  test

s ResNet- 1 1 0
 VGG- 1 6  7 3 . 2  70 . 4

1 ResNet- 1 0 1   76.4  73.8

0  2 0  40  6 0  8 0  1 00

layer index (original)
 Table  7 .  Obj ect  detection  mAP  (%)  on  the  PASCAL  VOC

3
 plain--20
 - 

plain 56
 2007/20 1 2  test  sets  using  baseline  Faster  R CNN.  See  also  Ta

ResNet-20

dts
 2
 ResNet-56
 ble  1 0  and  1 1   for  better  results .

ResNet- 1 1 0

1 metric  mAP @ . 5  mAP @ [ . 5 ,  . 95]

0  20  40  60  80  1 00
 VGG- 1 6  4 1 . 5  2 1 . 2

layer index (sorted by magnitude)
 -

Figure  7 .  Standard  deviations  (std)  of  layer  responses  on  CIFAR ResNet 1 0 1   48.4  27.2

1 0.  The  responses  are  the  outputs  of  each  3 × 3  layer,  after  BN  and
 Table  8 .  Obj ect  detection  mAP  (%)  on  the  COCO  validation  set

before  nonlinearity.  Top :  the  layers  are  shown  in  their  original
 using  baseline  Faster  R-CNN.  See  also  Table  9  for  better  results .

order.  Bottom :  the  responses  are  ranked  in  descending  order.

have  similar  training  error.  We  argue  that  this  is  because  of

networks  such  as  FitNet  [3 5]  and  Highway  [42]  (Table  6) ,
 overfitting .  The   1 202-layer  network  may  be  unnecessarily

yet  is  among  the  state-of-the-art  results  (6 .43 % ,  Table  6) .
 large  ( 1 9 .4M)  for  this  small  dataset.  Strong  regularization

such  as  maxout  [ 1 0]  or  dropout  [ 1 4]  is  applied  to  obtain  the

Analysis  of  Layer  Responses.  Fig .  7  shows  the  standard
 best  results  ( [ 1 0,  25 ,  24,  3 5 ] )  on  this  dataset.  In  this  paper,

deviations  (std)  of  the  layer  responses .  The  responses  are
 we  use  no  maxout/dro out  and   ust  sim l  im ose  re ular

p j p y p g

the  outputs  of  each  3 × 3  layer,  after  BN  and  before  other
 ization  via  deep  and  thin  architectures  by  design,  without

nonlinearity  (ReLU/addition) .  For  ResNets,  this  analy distracting  from  the  focus  on  the  difficulties  of  optimiza

sis  reveals  the  response  strength  of  the  residual  functions .
 tion.  But  combinin  with  stron er  re ularization  ma  im

g g g y

Fig .  7  shows  that  ResNets  have  generally  smaller  responses
 rove  results ,  which  we  will  stud  in  the  future.

p y

than  their  plain  counterparts .  These  results  support  our  ba

sic  motivation  (Sec. 3 . 1 )  that  the  residual  functions  might
 4.3.  Object  Detection  on  PASCAL  and  MS  COCO

be  generally  closer  to  zero  than  the  non-residual  functions .

 Our  method  has  good  generalization  performance  on

We  also  notice  that  the  deeper  ResNet  has  smaller  magni 

other  recognition  tasks .  Table  7  and  8  show  the  obj ect  de

tudes  of  responses,  as  evidenced  by  the  comparisons  among

- tection  baseline  results  on  PASCAL  VOC  2007  and  20 1 2

ResNet 20,  5 6,  and   1 1 0  in  Fig .  7 .  When  there  are  more
 - 

[5]  and  COCO  [26] .  We  adopt  Faster R  CNN  [32]  as  the  de

layers ,  an  individual  layer  of  ResNets  tends  to  modify  the

tection  method.  Here  we  are  interested  in  the  improvements

signal  les s .
 - -

of  replacing  VGG 1 6  [4 1 ]  with  ResNet 1 0 1 .  The  detection

Exploring  Over  1000  layers.  We  explore  an  aggressively
 implementation  (see  appendix)  of  using  both  models  is  the

deep  model  of  over   1 000  layers .  We  set  n  =  200  that
 same,  so  the  gains  can  only  be  attributed  to  better  networks .

leads  to  a   1 202-layer  network,  which  is  trained  as  described
 Most  remarkably,  on  the  challenging  COCO  dataset  we  ob

’

above.  Our  method  shows  no  optimization  difficulty,  and
 tain  a  6.0%  increase  in  COCO s  standard  metric  (mAP @ [ .5 ,

this  1 03
-layer  network  is  able  to  achieve  training  error
 . 95 ] ) ,  which  is  a  28 %  relative  improvement.  This  gain  is

< 0 . 1 %  (Fig .  6 ,  right) .  Its  test  error  is  still  fairly  good
 solely  due  to  the  learned  representations .

(7 . 93 % ,  Table  6) .
 B ased  on  deep  residual  nets ,  we  won  the   1 st  places  in

But  there  are  still  open  problems  on  such  aggressively
 several  tracks  in  ILSVRC  &  COCO  20 1 5  competitions :  Im

deep  models .  The  testing  result  of  this   1 202-layer  network
 ageNet  detection,  ImageNet  localization,  COCO  detection,

is  worse  than  that  of  our   1 1 0-layer  network,  although  both
 and  COCO  segmentation.  The  details  are  in  the  appendix.

8

´

References
 [28]  G.  Montu
far,  R.  Pascanu,  K.  Cho,  and  Y.  Bengio.  On  the  number  of

linear  regions  of  deep  neural  networks .  In  NIPS,  20 1 4 .

[ 1 ]  Y.  Bengio,  P.  Simard,  and  P.  Frasconi.  Learning  long-term  dependen

[29]  V.  Nair  and  G.  E.  Hinton.  Rectified  linear  units  improve  restricted

cies  with  gradient  descent  is  difficult.  IEEE  Transactions  on N  eural

boltzmann  machines .  In  ICML,  20 1 0.

Networks,  5 (2) : 1 57– 1 66,   1 994 .

[30]  F.  Perronnin  and  C .  Dance.  Fisher  kernels  on  visual  vocabularies  for

[2]  C .  M.  Bishop .  Neural  networks f  or p  attern  recognition.  Oxford

image  categorization.  In  CVPR,  2007 .

university  pres s ,   1 995 .

[3 1 ]  T.  Raiko,  H.  Valpola,  and  Y.  LeCun.  Deep  learning  made  easier  by

[3 ]  W.  L.  Briggs ,  S .  F.  McCormick,  et  al.  A M  ultigrid  Tutorial.  Siam,

linear  transformations  in  perceptrons .  In  AISTATS,  20 1 2.

2000 .
 -

[3 2]  S .  Ren,  K.  He,  R.  Girshick,  and  J.  Sun.  Faster  R CNN :  Towards

[4]  K.  Chatfield,  V.  Lempitsky,  A.  Vedaldi,  and  A.  Zisserman.  The  devil
 -

real time  obj ect  detection  with  region  proposal  networks .  In  NIPS,

is  in  the  details :  an  evaluation  of  recent  feature  encoding  methods .

20 1 5 .

In  BMVC,  20 1 1 .

[3 3 ]  S .  Ren,  K.  He,  R.  Girshick,  X.  Zhang,  and  J.  Sun.  Obj ect  detection

[5]  M.  Everingham,  L.  Van  Gool,  C .  K.  Williams,  J.  Winn,  and  A.  Zis

networks  on  convolutional  feature  maps .  arXiv: 1 504. 06066,  20 1 5 .

serman.  The  Pascal  Visual  Obj ect  Classes  (VOC)  Challenge.  IJCV,

– [34]  B .  D .  Ripley.  Pattern  recognition  and  neural  networks.  Cambridge

pages  3 03 3 3 8 ,  20 1 0 .

university  pres s ,   1 996 .

[6]  S .  Gidaris  and  N.  Komodakis .  Obj ect  detection  via  a  multi-region  &

[3 5 ]  A.  Romero,  N.  B allas ,  S .  E.  Kahou,  A.  Chas sang,  C .  Gatta,  and

semantic  segmentation-aware  cnn  model.  In  ICCV,  20 1 5 .

Y.  B engio .  Fitnets :  Hints  for  thin  deep  nets .  In  ICLR,  20 1 5 .

[7]  R.  Girshick.  Fast  R-CNN.  In  ICCV,  20 1 5 .

[3 6]  O .  Rus sakovsky,  J .  Deng,  H .  Su,  J .  Krause,  S .  S atheesh,  S .  Ma,

[8]  R.  Girshick,  J.  Donahue,  T.  Darrell,  and  J.  Malik.  Rich  feature  hier

Z.  Huang,  A.  Karpathy,  A.  Khosla,  M .  B ernstein,  et  al.  Imagenet

archies  for  accurate  obj ect  detection  and  semantic  segmentation.  In

large  scale  visual  recognition  challenge.  arXiv: 1409. 0575,  20 1 4 .

CVPR,  20 1 4.

[37]  A.  M .  S axe,  J.  L.  McClelland,  and  S .  Ganguli .  Exact  solutions  to

[9]  X.  Glorot  and  Y.  B engio .  Understanding  the  difficulty  of  training

the  nonlinear  dynamics  of  learning  in  deep  linear  neural  networks .

deep  feedforward  neural  networks .  In  AISTATS,  20 1 0.

arXiv: 1 31 2. 61 20,  20 1 3 .

[ 1 0]  I.  J.  Goodfellow,  D .  Warde-Farley,  M .  Mirza,  A.  Courville,  and
 -

[3 8]  N.  N.  Schraudolph.  Accelerated  gradient  descent  by  factor centering

Y.  Bengio.  Maxout  networks .  arXiv: 1 302. 4389,  20 1 3 .

decomposition.  Technical  report,   1 998 .

[ 1 1 ]  K.  He  and  J.  Sun.  Convolutional  neural  networks  at  constrained  time

[3 9]  N.  N.  Schraudolph.  Centering  neural  network  gradient  factors .  In

cost.  In  CVPR,  20 1 5 .
 –

Neural N  etworks:  Tricks  of  the  Trade,  pages  207 226 .  Springer,

[ 1 2]  K.  He,  X.  Zhang,  S .  Ren,  and  J.  Sun.  Spatial  pyramid  pooling  in  deep
 1 99 8 .

convolutional  networks  for  visual  recognition.  In  ECCV,  20 1 4.
 

[40]  P.  Sermanet,  D .  Eigen,  X.  Zhang,  M.  Mathieu,  R.  Fergus,  and  Y.  Le

[ 1 3 ]  K.  He,  X.  Zhang,  S .  Ren,  and  J.  Sun.  Delving  deep  into  rectifiers :
 Cun.  Overfeat:  Integrated  recognition,  localization  and  detection

Surpassing  human-level  performance  on  imagenet  classification.  In
 using  convolutional  networks .  In  ICLR,  20 1 4.

ICCV,  20 1 5 .

[4 1 ]  K.  Simonyan  and  A.  Zisserman.  Very  deep  convolutional  networks

[ 1 4]  G.  E.  Hinton,  N.  Srivastava,  A.  Krizhevsky,  I.  Sutskever,  and
 for  large- scale  image  recognition.  In  ICLR,  20 1 5 .

R.  R.  Salakhutdinov.  Improving  neural  networks  by  preventing  co

[42]  R.  K.  Srivastava,  K.  Greff,  and  J.  Schmidhuber.  Highway  networks .

adaptation  of  feature  detectors .  arXiv: 1 207. 0580,  20 1 2.

arXiv: 1 505. 00387,  20 1 5 .

[ 1 5]  S .  Hochreiter  and  J.  Schmidhuber.  Long  short-term  memory.  Neural

[43 ]  R.  K.  Srivastava,  K.  Greff,  and  J.  Schmidhuber.  Training  very  deep

computation,  9(8) : 1 73 5– 1 7 80,   1 997 .

networks .  1 507. 06228,  20 1 5 .

[ 1 6]  S .  Ioffe  and  C .  Szegedy.  B atch  normalization :  Accelerating  deep
 

[44]  C .  Szegedy,  W.  Liu,  Y.  Jia,  P.  Sermanet,  S .  Reed,  D .  Anguelov,  D .  Er

network  training  by  reducing  internal  covariate  shift.  In  ICML,  20 1 5 .
 

han,  V.  Vanhoucke,  and  A.  Rabinovich.  Going  deeper  with  convolu

[ 1 7]  H.  Jegou,  M .  Douze,  and  C .  Schmid.  Product  quantization  for  nearest
 tions .  In  CVPR,  20 1 5 .

neighbor  search.  TPAMI,  3 3 ,  20 1 1 .
 

[45 ]  R.  Szeliski .  Fast  surface  interpolation  using  hierarchical  basis  func

[ 1 8]  H.  Jegou,  F.  Perronnin,  M .  Douze,  J.  S anchez,  P.  Perez,  and
 tions .  TPAMI,   1 990 .

C .  Schmid.  Aggregating  local  image  descriptors  into  compact  codes .

[46]  R.  Szeliski .  Locally  adapted  hierarchical  basis  preconditioning .  In

TPAMI,  20 1 2.

SIGGRAPH,  2006.

[ 1 9]  Y.  Jia,  E.  Shelhamer,  J.  Donahue,  S .  Karayev,  J.  Long,  R.  Girshick,
 

[47]  T.  Vatanen,  T.  Raiko,  H.  Valpola,  and  Y.  LeCun.  Pushing  stochas

S .  Guadarrama,  and  T.  Darrell.  Caffe :  Convolutional  architecture  for
 - – 

tic  gradient  towards  second order  methods backpropagation  learn

fast  feature  embedding.  arXiv: 1408. 5093,  20 1 4.

ing  with  transformations  in  nonlinearities .  In  Neural I  nformation

[20]  A.  Krizhevsky.  Learning  multiple  layers  of  features  from  tiny  im Processin ,  20 1 3 .

g

ages .  Tech R  eport,  2009 .

[48]  A.  Vedaldi  and  B .  Fulkerson.  VLFeat:  An  open  and  portable  library

[2 1 ]  A.  Krizhevsky,  I.  Sutskever,  and  G.  Hinton.  Imagenet  classification
 of  com uter  vision  al orithms ,  2008 .

p g

with  deep  convolutional  neural  networks .  In  NIPS,  20 1 2.
 -

[49]  W.  Venables  and  B .  Ripley.  Modern  applied  statistics  with  s plus .

[22]  Y.  LeCun,  B .  B oser,  J.  S .  Denker,  D .  Henderson,  R.  E.  Howard,
 1 999 .

W.  Hubbard,  and  L.  D .  Jackel.  B ackpropagation  applied  to  hand 

[50]  M.  D .  Zeiler  and  R.  Fergus .  Visualizing  and  understanding  convolu

written  zip  code  recognition.  Neural  computation,   1 989 .

¨ tional  neural  networks .  In  ECCV,  20 1 4 .

[23 ]  Y.  LeCun,  L.  B ottou,  G.  B .  Orr,  and  K. -R.  Mu
 ller.  Efficient  backprop .

In  Neural N  etworks:  Tricks  of the  Trade,  pages  9–50.  Springer,   1 998 .

[24]  C . -Y.  Lee,  S .  Xie,  P.  Gallagher,  Z.  Zhang,  and  Z.  Tu.  Deeply

supervised  nets .  arXiv: 1409. 51 85,  20 1 4 .

[25]  M.  Lin,  Q.  Chen,  and  S .  Yan.  Network  in  network.  arXiv: 1 31 2. 4400,

20 1 3 .

[26]  T. -Y.  Lin,  M .  Maire,  S .  B elongie,  J.  Hays ,  P.  Perona,  D .  Ramanan,

´

P.  Dolla
r,  and  C .  L.  Zitnick.  Microsoft  COCO :  Common  obj ects  in

context.  In  ECCV.  20 1 4 .

[27]  J.  Long,  E.  Shelhamer,  and  T.  Darrell.  Fully  convolutional  networks

for  semantic  segmentation.  In  CVPR,  20 1 5 .

# 9

A.  Obj ect  Detection  Baselines
 8  images  (i. e. ,   1  per  GPU)  and  the  Fast  R-CNN  step  has  a

mini-batch  size  of   1 6  images .  The  RPN  step  and  Fast  R

In  this  section  we  introduce  our  detection  method  based
 

CNN  step  are  both  trained  for  240k  iterations  with  a  learn

on  the  baseline  Faster  R-CNN  [32]  system.  The  models  are

ing  rate  of  0 . 00 1  and  then  for  80k  iterations  with  0 . 000 1 .

initialized  by  the  ImageNet  classification  models,  and  then

Table  8  shows  the  results  on  the  MS  COCO  validation

fine-tuned  on  the  obj ect  detection  data.  We  have  experi -

set.  ResNet 1 0 1  has  a  6%  increase  of  mAP @ [ . 5 ,  . 95 ]  over

mented  with  ResNet-50/1 0 1  at  the  time  of  the  ILSVRC  &
 - 

VGG 1 6,  which  is  a  28 %  relative  improvement,  solely  con

COCO  20 1 5  detection  competitions .
 

tributed  by  the  features  learned  by  the  better  network.  Re

Unlike  VGG- 1 6  used  in  [3 2] ,  our  ResNet  has  no  hidden
 ’

“ markably,  the  mAP @ [ . 5 ,  . 95 ] s  absolute  increase  (6 .0%)  is

fc  layers .  We  adopt  the  idea  of   Networks  on  Conv  fea ’

” nearly  as  big  as  mAP @ . 5 s  (6 . 9 %) .  This  suggests  that  a

ture  maps  (NoC)  [3 3 ]  to  address  this  issue.  We  compute
 

deeper  network  can  improve  both  recognition  and  localiza

the  full-image  shared  conv  feature  maps  using  those  lay

tion .

ers  whose  strides  on  the  image  are  no  greater  than   1 6  pixels

(i. e . ,  conv 1 ,  conv2  x,  conv3  x,  and  conv4  x,  totally  9 1  conv

layers  in  ResNet- 1 0 1 ;  Table  1 ) .  We  consider  these  layers  as
 B.  Obj ect  Detection  Improvements

analogous  to  the   1 3  conv  layers  in  VGG- 1 6,  and  by  doing

For  completeness,  we  report  the  improvements  made  for

so,  both  ResNet  and  VGG- 1 6  have  conv  feature  maps  of  the

the  competitions .  These  improvements  are  based  on  deep

same  total  stride  ( 1 6  pixels) .  These  layers  are  shared  by  a

features  and  thus  should  benefit  from  residual  learning .

region  proposal  network  (RPN,  generating  300  proposals)

[32]  and  a  Fast  R-CNN  detection  network  [7] .  RoI  pool MS  COCO

ing  [7]  is  performed  before  conv5  1 .  On  this  RoI-pooled
 Box  refinement.   Our  box  refinement  partially  follows  the  it

feature,  all  layers  of  conv5  x  and  up  are  adopted  for  each
 erative  localization  in  [6] .  In  Faster  R-CNN,  the  final  output

region,  playing  the  roles  of  VGG- 1 6 ’ s  fc  layers .  The  final
 is  a  regres sed  box  that  is  different  from  its  proposal  box.  S o

classification  layer  is  replaced  by  two  sibling  layers  (classi for  inference,  we  pool  a  new  feature  from  the  regressed  box

fication  and  box  regression  [7] ) .
 and  obtain  a  new  classification  score  and  a  new  regressed

For  the  usage  of  BN  layers,  after  pre-training,  we  com box.  We  combine  these  300  new  predictions  with  the  orig

pute  the  BN  statistics  (means  and  variances)  for  each  layer
 inal  300  predictions .  Non-maximum  suppression  (NMS)  is

on  the  ImageNet  training  set.  Then  the  BN  layers  are  fixed
 applied  on  the  union  set  of  predicted  boxes  using  an  IoU

during  fine-tuning  for  obj ect  detection.  As  such,  the  BN
 threshold  of  0. 3  [8] ,  followed  by  box  voting  [6] .  B ox  re

layers  become  linear  activations  with  constant  offsets  and
 finement  improves  mAP  by  about  2  points  (Table  9) .

scales ,  and  BN  statistics  are  not  updated  by  fine-tuning .  We

Global  context.   We  combine  global  context  in  the  Fast

fix  the  BN  layers  mainly  for  reducing  memory  consumption
 - -

R CNN  step.  Given  the  full image  conv  feature  map,  we

in  Faster  R-CNN  training .

pool  a  feature  by  global  Spatial  Pyramid  Pooling  [ 1 2]  (with

PASCAL  VOC
 a  “single-level”  pyramid)  which  can  be  implemented  as

“ ” ’

Following  [7 ,  3 2] ,  for  the  PASCAL  VOC  2007  test  set,
 RoI  pooling  using  the  entire  image s  bounding  box  as  the

we  use  the  5k  trainval  images  in  VOC  2007  and   1 6k  train RoI.  This  pooled  feature  is  fed  into  the  post-RoI  layers  to

val  images  in  VOC  20 1 2  for  training  (“07+ 1 2”) .  For  the
 obtain  a  global  context  feature.  This  global  feature  is  con

PASCAL  VOC  20 1 2  test  set,  we  use  the   1 0k  trainval+test
 catenated  with  the  original  per-region  feature,  followed  by

images  in  VOC  2007  and   1 6k  trainval  images  in  VOC  20 1 2
 the  sibling  classification  and  box  regression  layers .  This

for  training  (“07++ 1 2”) .  The  hyper-parameters  for  train new  structure  is  trained  end-to-end.  Global  context  im

ing  Faster  R-CNN  are  the  same  as  in  [3 2] .  Table  7  shows
 proves  mAP @ . 5  by  about   1  point  (Table  9) .

the  results .  ResNet- 1 0 1  improves  the  mAP  by  > 3 %  over
 -

Multi scale  testing.   In  the  above,  all  results  are  obtained  by

VGG- 1 6 .  This  gain  is  solely  because  of  the  improved  fea - ’

single scale  training/testing  as  in  [3 2] ,  where  the  image s

tures  learned  by  ResNet.
 -

shorter  side  is  s  =  600  pixels .  Multi scale  training/testing

MS  COCO
 has  been  developed  in  [ 1 2,  7]  by  selecting  a  scale  from  a

The  MS  COCO  dataset  [26]  involves  80  obj ect  cate feature  pyramid,  and  in  [3 3 ]  by  using  maxout  layers .  In

gories .  We  evaluate  the  PASCAL  VOC  metric  (mAP   @ 
 our  current  implementation,  we  have  performed  multi-scale

IoU  =  0. 5)  and  the  standard  COCO  metric  (mAP   @  IoU  =
 testing  following  [3 3 ] ;  we  have  not  performed  multi-scale

. 5 : . 05 : . 95) .  We  use  the  80k  images  on  the  train  set  for  train training  because  of  limited  time.  In  addition,  we  have  per

ing  and  the  40k  images  on  the  val  set  for  evaluation.  Our
 formed  multi- scale  testing  only  for  the  Fast  R-CNN  step

detection  system  for  COCO  is  similar  to  that  for  PASCAL
 (but  not  yet  for  the  RPN  step) .  With  a  trained  model,  we

VOC.  We  train  the  COCO  models  with  an  8-GPU  imple compute  conv  feature  maps  on  an  image  pyramid,  where  the

mentation,  and  thus  the  RPN  step  has  a  mini-batch  size  of
 image ’ s  shorter  sides  are  s  ∈  { 200 ,  400 ,  600 ,  800 ,  1 000 } .

1 0

training  data  COCO  train  COCO  trainval

test  data  COCO  val  COCO  test-dev

mAP  @ . 5  @ [ . 5 ,  . 95 ]   @ . 5  @ [ . 5 ,  . 95 ]

baseline  Faster  R-CNN  (VGG- 1 6)  4 1 . 5  2 1 .2

baseline  Faster  R-CNN  (ResNet- 1 0 1 )  48 .4  27 .2

+box  refinement  49 .9  29 .9

+context  5 1 . 1   3 0 . 0  5 3 . 3  3 2 . 2

+multi- scale  testing  5 3 . 8  3 2 . 5  55.7  34.9

ensemble  59.0  37.4

Table  9 .  Obj ect  detection  improvements  on  MS  COCO  using  Faster  R-CNN  and  ResNet- 1 0 1 .

system  net  data  mAP  areo  bike  bird  boat  bottle  bus  car  cat  chair  cow  table  dog  horse  mbike  person  plant  sheep  sofa  train  tv

baseline  VGG- 1 6  07+ 1 2  7 3 . 2  76 . 5  79 . 0  70 . 9  65 . 5  5 2 . 1  8 3 . 1  84 . 7  8 6 .4  5 2 . 0  8 1 . 9  65 . 7  84 . 8  84 . 6  77 . 5  76 . 7  3 8 . 8  7 3 . 6  7 3 . 9  8 3 . 0  72 . 6

baseline  ResNet- 1 0 1   07+ 1 2  76 .4  79 . 8  80 . 7  76 . 2  6 8 . 3  5 5 . 9  8 5 . 1  8 5 . 3   89.8  5 6 . 7  87 . 8  69 .4  8 8 . 3  8 8 . 9  80 . 9  7 8 .4  4 1 . 7  7 8 . 6  79 . 8  8 5 . 3  72 . 0

baseline+++  ResNet- 1 0 1   COCO+07+ 1 2  85.6  90.0  89.6  87.8  80.8  76. 1  89.9  89.9  89 . 6  75.5  90.0  80.7  89.6  90.3  89. 1  88.7  65.4  88. 1  85.6  89.0  86.8

Table   1 0.  Detection  results  on  the  PASCAL  VOC  2007  test  set.  The  baseline  is  the  Faster  R-CNN  system.  The  system  “baseline+++”

include  box  refinement,  context,  and  multi- scale  testing  in  Table  9 .

system  net  data  mAP  areo  bike  bird  boat  bottle  bus  car  cat  chair  cow  table  dog  horse  mbike  person  plant  sheep  sofa  train  tv

baseline  VGG- 1 6  07++ 1 2  70 .4  84 . 9  79 . 8  74 . 3  5 3 . 9  49 . 8  77 . 5  75 . 9  8 8 . 5  45 . 6  77 . 1  5 5 . 3  8 6 . 9  8 1 . 7  80 . 9  79 . 6  40 . 1  72 . 6  60 . 9  8 1 . 2  6 1 . 5

baseline  ResNet- 1 0 1   07++ 1 2  7 3 . 8   8 6 . 5  8 1 . 6  77 . 2  5 8 . 0  5 1 . 0  7 8 . 6  76 . 6  93 . 2  48 . 6  80 .4  5 9 . 0  92 . 1  8 5 . 3  84 . 8  80 . 7  48 . 1  77 . 3  66 . 5  84 . 7  65 . 6

baseline+++  ResNet- 1 0 1   COCO+07++ 1 2  83.8  92. 1  88.4  84.8  75.9  71 .4  86.3  87.8  94.2  66.8  89.4  69.2  93.9  91 .9  90.9  89.6  67.9  88.2  76.8  90.3  80.0

Table   1 1 .  Detection  results  on  the  PASCAL  VOC  20 1 2  test  set  (ht t p : / / h o s t . r ob o t s . o x . a c . u k : 8 0 8 0 / l e a de rb o a r d /

di s p l ay l b . php ? ch a l l e n ge i d= 1 1 & c omp i d= 4 ) .  The  baseline  is  the  Faster  R-CNN  system.  The  system  “baseline+++”  include

box  refinement,  context,  and  multi- scale  testing  in  Table  9 .

We  select  two  adj acent  scales  from  the  pyramid  following
 val2  test

[3 3 ] .  RoI  pooling  and  subsequent  layers  are  performed  on
 GoogLeNet  [44]  (ILSVRC ’ 1 4)  -  43 .9

the  feature  maps  of  these  two  scales  [3 3 ] ,  which  are  merged
 our  single  model  (ILSVRC ’ 1 5)  60.5  5 8 . 8

by  maxout  as  in  [3 3 ] .  Multi- scale  testing  improves  the  mAP
 ’

our  ensemble  (ILSVRC 1 5)  63.6  62. 1

by  over  2  points  (Table  9) .

Table   1 2.  Our  results  (mAP,  %)  on  the  ImageNet  detection  dataset.

Using  validation  data.   Next  we  use  the  80k+40k  trainval  set
 Our  detection  system  is  Faster  R-CNN  [32]  with  the  improvements

for  training  and  the  20k  test-dev  set  for  evaluation.  The  test in  Table  9,  using  ResNet- 1 0 1 .

dev  set  has  no  publicly  available  ground  truth  and  the  result

is  reported  by  the  evaluation  server.  Under  this  setting,  the

we  achieve  85 .6%  mAP  on  PASCAL  VOC  2007  (Table  1 0)

results  are  an  mAP @ . 5  of  5 5 .7 %  and  an  mAP @ [ . 5 ,  . 95 ]  of
 6

and  83 . 8 %  on  PASCAL  VOC  20 1 2  (Table  1 1 )
 .  The  result

34 . 9 %  (Table  9) .  This  is  our  single-model  result.
 

on  PASCAL  VOC  20 1 2  is   1 0  points  higher  than  the  previ

Ensemble.   In  Faster  R-CNN,  the  system  is  designed  to  learn
 ous  state-of-the-art  result  [6] .

region  proposals  and  also  obj ect  classifiers ,  so  an  ensemble

can  be  used  to  boost  both  tasks .  We  use  an  ensemble  for
 ImageNet  Detection

ro osin  re ions,  and  the  union  set  of   ro osals  are   ro The  ImageNet  Detection  (DET)  task  involves  200  obj ect

p p g g p p p

cessed  b  an  ensemble  of   er-re ion  classifiers .  Table  9
 categories .  The  accuracy  is  evaluated  by  mAP @ . 5 .  Our

y p g

shows  our  result  based  on  an  ensemble  of  3  networks .  The
 obj ect  detection  algorithm  for  ImageNet  DET  is  the  same

mAP  is  59 .0%  and  37 .4%  on  the  test-dev  set.  This  result
 as  that  for  MS  COCO  in  Table  9 .  The  networks  are  pre

won  the  1 st   lace  in  the  detection  task  in  COCO  201 5. 
 trained  on  the   1 000-class  ImageNet  classification  set,  and

p

are  fine-tuned  on  the  DET  data.  We  split  the  validation  set

PASCAL  VOC
 into  two  parts  (val 1/val2)  following  [8] .  We  fine-tune  the

We  revisit  the  PASCAL  VOC  dataset  based  on  the  above
 detection  models  using  the  DET  training  set  and  the  val 1

model.  With  the  single  model  on  the  COCO  dataset  (55 .7 %
 set.  The  val2  set  is  used  for  validation.  We  do  not  use  other

mAP @ .5  in  Table  9) ,  we  fine-tune  this  model  on  the  PAS  ILSVRC  20 1 5  data.  Our  single  model  with  ResNet- 1 0 1  has

CAL  VOC  sets.  The  improvements  of  box  refinement,  con 6

ht t p : / / h o s t . r ob ot s . o x . a c . u k : 8 0 8 0 / an o nymou s / 3 O J 4 O J . html ,

text,  and  multi- scale  testing  are  also  adopted.  B y  doing  so
 submitted  on  20 1 5- 1 1 -26.

1 1

LOC 
 LOC 
 LOC  error
 classification 
 top-5  LOC  error
 top-5  localization  err

method 
 network   testing   on  GT  CLS 
 network 
 on  predicted  CLS 
 method  val  test

VGG ’ s  [4 1 ]   VGG- 1 6   1 -crop   3 3 . 1  [4 1 ] 
 ’

RPN   ResNet- 1 0 1   1 -crop   1 3 .3 
 OverFeat  [40]  (ILSVRC 1 3 )  3 0 . 0  29 . 9

RPN   ResNet- 1 0 1   dense   1 1 .7 
 GoogLeNet  [44]  (ILSVRC ’ 1 4)  -  26 .7

RPN   ResNet- 1 0 1   dense   ResNet- 1 0 1   1 4.4 
 VGG  [4 1 ]  (ILSVRC ’ 1 4)  26 . 9  25 . 3

RPN+RCNN   ResNet- 1 0 1   dense   ResNet- 1 0 1   10.6 
 ours  (ILSVRC ’ 1 5)  8.9  9.0

RPN+RCNN   ensemble   dense   ensemble   8.9

Table   1 4.  Comparisons  of  localization  error  (%)  on  the  ImageNet

Table   1 3 .  Localization  error  (%)  on  the  ImageNet  validation.  In

“ ” dataset  with  state-of-the-art  methods .

the  column  of   LOC  error  on  GT  class  ( [4 1 ] ) ,  the  ground  truth

clas s  is  used.  In  the  “testing”  column,  “ 1 -crop”  denotes  testing

on  a  center  crop  of  224 × 224  pixels,  “dense”  denotes  dense  (fully
 ports  a  center-crop  error  of  3 3 . 1 %  (Table  1 3 )  using  ground

convolutional)  and  multi-scale  testing.
 truth  classes .  Under  the  same  setting,  our  RPN  method  us

ing  ResNet- 1 0 1  net  significantly  reduces  the  center-crop  er

5 8 . 8 %  mAP  and  our  ensemble  of  3  models  has  62. 1 %  mAP

on  the  DET  test  set  (Table  1 2) .  This  result  won  the  1 st p  lace

in  the I  mageNet  detection  task  in I  LSVRC  201 5,  surpassing

the  second  place  by  8.5  points  (absolute) .

ror  to   1 3 . 3 % .  This  comparison  demonstrates  the  excellent

performance  of  our  framework.  With  dense  (fully  convolu

tional)  and  multi- scale  testing,  our  ResNet- 1 0 1  has  an  error

of   1 1 .7 %  using  ground  truth  classes .  Using  ResNet- 1 0 1  for

predicting  classes  (4 . 6 %  top-5  classification  error,  Table  4) ,

the  top-5  localization  error  is   1 4 .4 % .

C.  ImageNet  Localization
 The  above  results  are  only  based  on  the  proposal  network

The  ImageNet  Localization  (LOC)  task  [3 6]  requires  to
 (RPN)  in  Faster  R-CNN  [32] .  One  may  use  the  detection

classify  and  localize  the  obj ects .  Following  [40,  4 1 ] ,  we
 network  (Fast  R-CNN  [7] )  in  Faster  R-CNN  to  improve  the

assume  that  the  ima e-level  classifiers  are  first  ado ted  for
 results .  But  we  notice  that  on  this  dataset,  one  image  usually

g p

predicting  the  class  labels  of  an  image,  and  the  localiza contains  a  single  dominate  obj ect,  and  the  proposal  regions

tion  algorithm  only  accounts  for  predicting  bounding  boxes
 highly  overlap  with  each  other  and  thus  have  very  similar

based  on  the   redicted  classes .  We  ado t  the  “ er-class  re RoI-pooled  features .  As  a  result,  the  image-centric  training

p p p

gression”  (PCR)  strategy  [40,  4 1 ] ,  learning  a  bounding  box
 of  Fast  R-CNN  [7]  generates  samples  of  small  variations ,

re ressor  for  each  class .  We   re-train  the  networks  for  Im which  may  not  be  desired  for  stochastic  training .  Motivated

g p

ageNet  classification  and  then  fine-tune  them  for  localiza by  this,  in  our  current  experiment  we  use  the  original  R

tion.  We  train  networks  on  the   rovided   1 000-class  Ima CNN  [8]  that  is  RoI-centric,  in  place  of  Fast  R-CNN.

p

eNet  trainin  set.
 Our  R-CNN  implementation  is  as  follows .  We  apply  the

g g

 per-class  RPN  trained  as  above  on  the  training  images  to

Our  localization  algorithm  is  based  on  the  RPN  frame

predict  bounding  boxes  for  the  ground  truth  class .  These

work  of  [3 2]  with  a  few  modifications .  Unlike  the  way  in

- predicted  boxes  play  a  role  of  class-dependent  proposals .

[3 2]  that  is  category agnostic,  our  RPN  for  localization  is

-  For  each  training  image,  the  highest  scored  200  proposals

designed  in  a  per class  form.  This  RPN  ends  with  two  sib

are  extracted  as  training  samples  to  train  an  R-CNN  classi

ling   1 × 1  convolutional  layers  for  binary  classification  (cls)

fier.  The  image  region  is  cropped  from  a  proposal,  warped

and  box  regres sion  (reg) ,  as  in   [3 2] .  The  cls  and  reg  layers

-  to  224 × 224  pixels ,  and  fed  into  the  classification  network

are  both  in  a  per class  from,  in  contrast  to  [3 2] .  Specifi

- as  in  R-CNN  [8] .  The  outputs  of  this  network  consist  of  two

cally,  the  cls  layer  has  a   1 000 d  output,  and  each  dimension

 sibling  fc  layers  for  cls  and  reg,  also  in  a  per-clas s  form.

is  binary  logistic  regression  for  predicting  being  or  not  be

- This  R-CNN  network  is  fine-tuned  on  the  training  set  us

ing  an  obj ect  clas s ;  the  reg  layer  has  a   1 000 × 4 d  output

ing  a  mini-batch  size  of  25 6  in  the  RoI-centric  fashion.  For

consisting  of  box  regres sors  for   1 000  clas ses .  As  in   [3 2] ,

testing,  the  RPN  generates  the  highest  scored  200  proposals

our  bounding  box  regression  is  with  reference  to  multiple

- “ ” for  each  predicted  class ,  and  the  R-CNN  network  is  used  to

translation invariant   anchor  boxes  at  each  position.
 ’

update  these  proposals  scores  and  box  positions .

As  in  our  ImageNet  classification  training  (Sec .  3 .4) ,  we

This  method  reduces  the  top-5  localization  error  to

randomly  sample  224 × 224  crops  for  data  augmentation.

- - 1 0 . 6 %  (Table  1 3 ) .  This  is  our  single-model  result  on  the

We  use  a  mini batch  size  of  256  images  for  fine tuning .  To

 validation  set.  Using  an  ensemble  of  networks  for  both  clas

avoid  negative  samples  being  dominate,  8  anchors  are  ran

sification  and  localization,  we  achieve  a  top-5  localization

domly  sampled  for  each  image,  where  the  sampled  positive

error  of  9 .0%  on  the  test  set.  This  number  significantly  out

and  negative  anchors  have  a  ratio  of   1 : 1   [3 2] .  For  testing,

- performs  the  ILSVRC   1 4  results  (Table  1 4) ,  showing  a  64%

the  network  is  applied  on  the  image  fully convolutionally.

relative  reduction  of  error.  This  result  won  the  1 st p  lace  in

Table  1 3  compares  the  localization  results .  Following

“ ” the I  mageNet  localization  task  in I  LSVRC  201 5.

[4 1 ] ,  we  first  perform   oracle  testing  using  the  ground  truth

class  as  the  classification  prediction.  VGG’ s  paper  [4 1 ]  re

1 2
