Published  as  a  conference  paper  at  ICLR  20 1 5

## NEURAL  MACHINE  TRANSLATION

## B Y  JOINTLY  LEARNING  TO  ALIGN  AND  TRANSLATE

Dzmitry  Bahdanau

Jacobs  University  Bremen,  Germany

KyungHyun  Cho  Yoshua  Bengio∗

´ ´

Universite   de  Montr e
 al

# 6

# 1

AB STRACT

# 0

# 2

Neural  machine  translation  is  a  recently  proposed  approach  to  machine  transla

y tion.  Unlike  the  traditional  statistical  machine  translation,  the  neural  machine

# a

translation  aims  at  building  a  single  neural  network  that  can  be j  ointly  tuned  to

M maximize  the  translation  performance.  The  models  proposed  recently  for  neu

ral  machine  translation  often  belong  to  a  family  of  encoder–decoders  and  encode

91 a  source  sentence  into  a  fixed-length  vector  from  which  a  decoder  generates  a

translation.  In  this  paper,  we  conj ecture  that  the  use  of  a  fixed-length  vector  is  a

] bottleneck  in  improving  the  performance  of  this  basic  encoder–decoder  architec

L ture,  and  propose  to  extend  this  by  allowing  a  model  to  automatically  (soft-)search

C for  parts  of  a  source  sentence  that  are  relevant  to  predicting  a  target  word,  without

. having  to  form  these  parts  as  a  hard  segment  explicitly.  With  this  new  approach,

# s

c we  achieve  a  translation  performance  comparable  to  the  existing  state-of-the-art

[ phrase-based  system  on  the  task  of  English-to-French  translation.  Furthermore,

qualitative  analysis  reveals  that  the  (soft-)alignments  found  by  the  model  agree

7 well  with  our  intuition.

# v

# 3

# 7

1  INTRODUCTION

# 4

# 0

. Neural  machine  translation  is  a  newly  emerging  approach  to  machine  translation,  recently  proposed

9 by  Kalchbrenner  and  Blunsom  (20 1 3) ,  Sutskever  et  al.   (20 1 4)  and  Cho  et  al.   (20 1 4b) .  Unlike  the

0 traditional  phrase-based  translation  system  (see,  e.g . ,  Koehn  et  al. ,  2003)  which  consists  of  many

41 small  sub-components  that  are  tuned  separately,  neural  machine  translation  attempts  to  build  and

: train  a  single,  large  neural  network  that  reads  a  sentence  and  outputs  a  correct  translation.

# v

i Most  of  the  proposed  neural  machine  translation  models  belong  to  a  family  of  encoder–

X decoders  (Sutskever  et  al. ,  20 1 4 ;  Cho  et  al. ,  20 1 4a) ,  with  an  encoder  and  a  decoder  for  each  lan

r guage,  or  involve  a  language-specific  encoder  applied  to  each  sentence  whose  outputs  are  then  com

a pared  (Hermann  and  Blunsom,  20 1 4) .  An  encoder  neural  network  reads  and  encodes  a  source  sen

tence  into  a  fixed-length  vector.  A  decoder  then  outputs  a  translation  from  the  encoded  vector.  The

whole  encoder–decoder  system,  which  consists  of  the  encoder  and  the  decoder  for  a  language  pair,

is j  ointly  trained  to  maximize  the  probability  of  a  correct  translation  given  a  source  sentence.

A  potential  issue  with  this  encoder–decoder  approach  is  that  a  neural  network  needs  to  be  able  to

compress  all  the  necessary  information  of  a  source  sentence  into  a  fixed-length  vector.  This  may

make  it  difficult  for  the  neural  network  to  cope  with  long  sentences ,  especially  those  that  are  longer

than  the  sentences  in  the  training  corpus .  Cho  et  al.   (20 1 4b)  showed  that  indeed  the  performance  of

a  basic  encoder–decoder  deteriorates  rapidly  as  the  length  of  an  input  sentence  increases .

In  order  to  address  this  issue,  we  introduce  an  extension  to  the  encoder–decoder  model  which  learns

to  align  and  translate j  ointly.  Each  time  the  proposed  model  generates  a  word  in  a  translation,  it

(soft-) searches  for  a  set  of  positions  in  a  source  sentence  where  the  most  relevant  information  is

concentrated.  The  model  then  predicts  a  target  word  based  on  the  context  vectors  associated  with

these  source  positions  and  all  the  previous  generated  target  words .

∗

CIFAR  Senior  Fellow

1

Published  as  a  conference  paper  at  ICLR  20 1 5

The  most  important  distinguishing  feature  of  this  approach  from  the  basic  encoder–decoder  is  that

it  does  not  attempt  to  encode  a  whole  input  sentence  into  a  single  fixed-length  vector.  Instead,  it  en

codes  the  input  sentence  into  a  sequence  of  vectors  and  chooses  a  subset  of  these  vectors  adaptively

while  decoding  the  translation.  This  frees  a  neural  translation  model  from  having  to  squash  all  the

information  of  a  source  sentence,  regardless  of  its  length,  into  a  fixed-length  vector.  We  show  this

allows  a  model  to  cope  better  with  long  sentences .

In  this  paper,  we  show  that  the  proposed  approach  of j  ointly  learning  to  align  and  translate  achieves

significantly  improved  translation  performance  over  the  basic  encoder–decoder  approach.  The  im

provement  is  more  apparent  with  longer  sentences,  but  can  be  observed  with  sentences  of  any

length.  On  the  task  of  English-to-French  translation,  the  proposed  approach  achieves ,  with  a  single

model,  a  translation  performance  comparable,  or  close,  to  the  conventional  phrase-based  system.

Furthermore,  qualitative  analysis  reveals  that  the  proposed  model  finds  a  linguistically  plausible

(soft-)alignment  between  a  source  sentence  and  the  corresponding  target  sentence.

2  BACKGROUND :  NEURAL  MACHINE  TRANSLATION

From  a  probabilistic  perspective,  translation  is  equivalent  to  finding  a  target  sentence  y  that  max

imizes  the  conditional  probability  of  y  given  a  source  sentence  x,  i . e. ,  arg  maxy  p (y  |   x) .  In

neural  machine  translation,  we  fit  a  parameterized  model  to  maximize  the  conditional  probability

of  sentence  pairs  using  a  parallel  training  corpus .  Once  the  conditional  distribution  is  learned  by  a

translation  model,  given  a  source  sentence  a  corresponding  translation  can  be  generated  by  searching

for  the  sentence  that  maximizes  the  conditional  probability.

Recently,  a  number  of  papers  have  proposed  the  use  of  neural  networks  to  directly  learn  this  condi

tional  distribution  (see,  e. g . ,  Kalchbren˜ner  and  B lunsom,  20 1 3 ;  Cho  et  al. ,  20 1 4a;  Sutskever  et  al. ,

20 1 4 ;  Cho  et  al. ,  20 1 4b ;  Forcada  and  N
 eco,   1 997) .  This  neural  machine  translation  approach  typ-

ically  consists  of  two  components ,  the  first  of  which  encodes  a  source  sentence  x  and  the  second

decodes  to  a  target  sentence  y .  For  instance,  two  recurrent  neural  networks  (RNN)  were  used  by

(Cho  et  al. ,  20 1 4a)  and  (Sutskever  et  al. ,  20 1 4)  to  encode  a  variable-length  source  sentence  into  a

fixed-length  vector  and  to  decode  the  vector  into  a  variable-length  target  sentence.

Despite  being  a  quite  new  approach,  neural  machine  translation  has  already  shown  promising  results .

Sutskever  et  al.   (20 1 4)  reported  that  the  neural  machine  translation  based  on  RNNs  with  long  short

term  memory  (LSTM)  units  achieves  close  to  the  state-of-the-art  performance  of  the  conventional

phrase-based  machine  translation  system  on  an  English-to-French  translation  task. 1  Adding  neural

components  to  existing  translation  systems ,  for  instance,  to  score  the  phrase  pairs  in  the  phrase

table  (Cho  et  al. ,  20 1 4a)  or  to  re-rank  candidate  translations  (Sutskever  et  al. ,  20 1 4) ,  has  allowed  to

surpass  the  previous  state-of-the-art  performance  level.

2 . 1  RNN  ENCODER–DECODER

Here,  we  describe  briefly  the  underlying  framework,  called  RNN  Encoder–Decoder,  proposed  by

Cho  et  al.   (20 1 4a)  and  Sutskever  et  al.   (20 1 4)  upon  which  we  build  a  novel  architecture  that  learns

to  align  and  translate  simultaneously.

In  the  Encoder–Decoder  framework,  an  encoder  reads  the  input  sentence,  a  sequence  of  vectors

x  =   ( x 1 ,  ·   ·   ·   ,  xTx
 ) ,  into  a  vector  c.
2  The  most  common  approach  is  to  use  an  RNN  such  that

ht  =  f  ( x t ,  ht − 1 )   ( 1 )

and

c  =   q  ( { h 1 ,   ·   ·   ·   ,   h Tx 
 } ) ,

where  ht  ∈  R
n  is  a  hidden  state  at  time  t,  and  c  is  a  vector  generated  from  the  sequence  of  the

hidden  states .  f  and  q  are  some  nonlinear  functions .  Sutskever  et  al.   (20 1 4)  used  an  LSTM  as  f  and

q  ( { h 1 ,   ·   ·   ·   ,  hT  } )  =  hT  ,  for  instance .

1  We  mean  by  the  state-of-the-art  performance,  the  performance  of  the  conventional  phrase-based  system

without  using  any  neural  network-based  component.

2  Although  most  of  the  previous  works  (see,  e. g . ,  Cho  et  al. ,  20 1 4a;  Sutskever  et  al. ,  20 1 4 ;  Kalchbrenner  and

Blunsom,  20 1 3 )  used  to  encode  a  variable-length  input  sentence  into  a  fixed-length  vector,  it  is  not  necessary,

and  even  it  may  be  beneficial  to  have  a  variable-length  vector,  as  we  will  show  later.

2

Published  as  a  conference  paper  at  ICLR  20 1 5

The  decoder  is  often  trained  to  predict  the  next  word  yt
0  given  the  context  vector  c  and  all  the

previously  predicted  words  { y1 ,  ·  ·  ·  ,  yt
0 − 1 } .  In  other  words ,  the  decoder  defines  a  probability  over

the  translation  y  by  decomposing  the j  oint  probability  into  the  ordered  conditionals :

T

p ( y )  =  Y p ( yt  |   { y 1 ,   ·   ·   ·   ,  yt − 1 }   ,  c) ,   (2)

t = 1

where  y  =
  y1 ,  ·  ·  ·  ,  yTy
  .  With  an  RNN,  each  conditional  probability  is  modeled  as

p ( yt  |   { y 1 ,   ·   ·   ·   ,  yt − 1 }   ,  c)  =  g ( yt − 1 ,  s t ,  c) ,   ( 3 )

where  g  is  a  nonlinear,  potentially  multi-layered,  function  that  outputs  the  probability  of  yt ,  and  st  is

the  hidden  state  of  the  RNN.  It  should  be  noted  that  other  architectures  such  as  a  hybrid  of  an  RNN

and  a  de-convolutional  neural  network  can  be  used  (Kalchbrenner  and  Blunsom,  20 1 3) .

3  LEARNING  TO  ALIGN  AND  TRANSLATE

In  this  section,  we  propose  a  novel  architecture  for  neural  machine  translation.  The  new  architecture

consists  of  a  bidirectional  RNN  as  an  encoder  (Sec .  3 . 2)  and  a  decoder  that  emulates  searching

through  a  source  sentence  during  decoding  a  translation  (Sec .  3 . 1 ) .

3 . 1  DECODER :  GENERAL  DES CRIPTION

In  a  new  model  architecture,  we  define  each  conditional  probability

in  Eq .  (2)  as : 
 yt-  1 yt

p ( yi
 | y 1 ,   .   .   .   ,  yi − 1 ,   x )  =  g ( yi − 1 ,  s i
 ,  ci ) ,   (4)
 s t- 1  s t

where  si
 is  an  RNN  hidden  state  for  time  i ,  computed  by

s i  =  f ( s i − 1 ,  yi − 1 ,  ci ) . 
 
+

It  should  be  noted  that  unlike  the  existing  encoder–decoder  ap αt, 1
 αt,T

α t , 2  α t , 3

proach  (see  Eq.  (2)) ,  here  the  probability  is  conditioned  on  a  distinct

context  vector  ci  for  each  target  word  yi
 .

h 1  h 2  h 3  h T

The  context  vector  ci  depends  on  a  sequence  of  annotations

( h 1 ,  ·   ·   ·   ,  hTx
 )   to  which  an  encoder  maps  the  input  sentence .  Each

annotation  hi  contains  information  about  the  whole  input  sequence
 h 1  h 2  h 3  hT

with  a  strong  focus  on  the  parts  surrounding  the  i-th  word  of  the

input  sequence.  We  explain  in  detail  how  the  annotations  are  com x 1 x2  x3  xT

puted  in  the  next  section.

Figure   1 :  The  graphical  illus

The  context  vector  ci
 is ,  then,  computed  as  a  weighted  sum  of  these
 tration  of  the  proposed  model

annotations  hi
 :
 trying  to  generate  the  t-th  tar

Tx
 
 get  word  yt  given  a  source

ci  =
 α i h   .   (5 )
 s entence  ( x 1 ,  x 2 ,   .   .   .   ,  x T  ) .

X j j

j = 1

The  weight  αij  of  each  annotation  hj  is  computed  by

exp   ( eij  )

α ij  =
 Tx
 ,   ( 6 )

P k = 1  exp   ( eik )

where

eij  =  a ( s i − 1 ,  hj  )

is  an  alignment  model  which  scores  how  well  the  inputs  around  position  j  and  the  output  at  position

i  match.  The  score  is  based  on  the  RNN  hidden  state  si − 1  (j ust  before  emitting  yi
 ,  Eq.  (4))  and  the

j -th  annotation  hj  of  the  input  sentence.

We  parametrize  the  alignment  model  a  as  a  feedforward  neural  network  which  is j  ointly  trained  with

all  the  other  components  of  the  proposed  system.  Note  that  unlike  in  traditional  machine  translation,

3

Published  as  a  conference  paper  at  ICLR  20 1 5

the  alignment  is  not  considered  to  be  a  latent  variable.  Instead,  the  alignment  model  directly  com

putes  a  soft  alignment,  which  allows  the  gradient  of  the  cost  function  to  be  backpropagated  through.

This  gradient  can  be  used  to  train  the  alignment  model  as  well  as  the  whole  translation  model j  ointly.

We  can  understand  the  approach  of  taking  a  weighted  sum  of  all  the  annotations  as  computing  an

expected  annotation,  where  the  expectation  is  over  possible  alignments .  Let  αij  be  a  probability  that

the  target  word  yi
 is  aligned  to,  or  translated  from,  a  source  word  xj  .  Then,  the  i -th  context  vector

ci
 is  the  expected  annotation  over  all  the  annotations  with  probabilities  αij  .

The  probability  αij  ,  or  its  associated  energy  eij  ,  reflects  the  importance  of  the  annotation  hj  with

respect  to  the  previous  hidden  state  si − 1  in  deciding  the  next  state  si  and  generating  yi
 .  Intuitively,

this  implements  a  mechanism  of  attention  in  the  decoder.  The  decoder  decides  parts  of  the  source

sentence  to  pay  attention  to .  By  letting  the  decoder  have  an  attention  mechanism,  we  relieve  the

encoder  from  the  burden  of  having  to  encode  all  information  in  the  source  sentence  into  a  fixed

length  vector.  With  this  new  approach  the  information  can  be  spread  throughout  the  sequence  of

annotations,  which  can  be  selectively  retrieved  by  the  decoder  accordingly.

3 . 2  ENCODER :  B IDIRECTIONAL  RNN  FOR  ANNOTATING  S EQUENCES

The  usual  RNN,  described  in  Eq.  ( 1 ) ,  reads  an  input  sequence  x  in  order  starting  from  the  first

symbol  x 1  to  the  last  one  xTx
 .  However,  in  the  proposed  scheme,  we  would  like  the  annotation

of  each  word  to  summarize  not  only  the  preceding  words,  but  also  the  following  words .  Hence,

we  propose  to  use  a  bidirectional  RNN  (BiRNN,  Schuster  and  Paliwal,   1 997) ,  which  has  been

succes sfully  used  recently  in  speech  recognition  (see,  e . g . ,  Graves  et  al. ,  20 1 3 ) .

’ →−

A  BiRNN  consists  of  forward  and  backward  RNN s .  The  forward  RNN
 f  reads  the  input  sequence

→− →−

as  it  is  ordered  (from  x 1  to  xTx
 )  and  calculates  a  sequence  of forward  hidden  states  (
 h  1 ,  ·   ·   ·   ,
 h  Tx
 ) .

←−

The  backward  RNN
 f  reads  the  sequence  in  the  reverse  order  (from  xTx
 to  x 1 ) ,  resulting  in  a

←−
 ←−

sequence  of  backward  hidden  states  (
 h  1 ,  ·   ·   ·   ,
 h  Tx
 ) .

→−

We  obtain  an  annotation  for  each  word  xj  by  concatenating  the  forward  hidden  state
 h  j  and  the

←−
 →− ←−
 >

h i

backward  one
 h  j  ,  i . e . ,  hj  =
 h
 j>
 
 ; 
 h
 j>
 
 .  In  this  way,  the  annotation  hj  contains  the  summaries

of  both  the  preceding  words  and  the  following  words .  Due  to  the  tendency  of  RNNs  to  better

represent  recent  inputs ,  the  annotation  hj  will  be  focused  on  the  words  around  xj  .  This  sequence

of  annotations  is  used  by  the  decoder  and  the  alignment  model  later  to  compute  the  context  vector

(Eqs .  (5 )–(6)) .

See  Fig .   1  for  the  graphical  illustration  of  the  proposed  model.

4  EXPERIMENT  S ETTINGS

We  evaluate  the  proposed  approach  on  the  task  of  English-to-French  translation.  We  use  the  bilin

gual,  parallel  corpora  provided  by  ACL  WMT  ’ 1 4.3  As  a  comparison,  we  also  report  the  perfor

mance  of  an  RNN  Encoder–Decoder  which  was  proposed  recently  by  Cho  et  al.   (20 1 4a) .  We  use

the  same  training  procedures  and  the  same  dataset  for  both  models .4

4 . 1  DATAS ET

WMT  ’ 1 4  contains  the  following  English-French  parallel  corpora:  Europarl  (6 1 M  words) ,  news

commentary  (5 .5M),  UN  (42 1 M)  and  two  crawled  corpora  of  90M  and  272.5M  words  respectively,

totaling  850M  words .  Following  the  procedure  described  in  Cho  et  al.   (20 1 4a) ,  we  reduce  the  size  of

the  combined  corpus  to  have  348M  words  using  the  data  selection  method  by  Axelrod  et  al.   (20 1 1 ) .5

We  do  not  use  any  monolingual  data  other  than  the  mentioned  parallel  corpora,  although  it  may  be

possible  to  use  a  much  larger  monolingual  corpus  to  pretrain  an  encoder.  We  concatenate  news-test

3  ht t p : / / www . s t atmt . o rg / wmt 1 4 / t r an s l at i o n - t a s k . html

4
 Implementations  are  available  at  ht t p s : / / g i t hub . c om / l i s a - g r o un dh o g / G r o un dH o g.

5  Available  online  at  ht t p : / / www- l i um . un i v- l eman s . f r / ˜ s chwe n k / c s lm_ j o i nt_p ap e r / .

4

Published  as  a  conference  paper  at  ICLR  20 1 5

3 0

2 5

e 2
 0 
 Figure  2 :  The  BLEU  scores

# r

oc of  the  generated  translations

# s

1 5 
 on  the  test  set  with  respect

# U

E to  the  lengths  of  the  sen

LB 1 0 
 RNNsearch-50 
 tences .  The  results  are  on

RNNsearch-30 
 the  full  test  set  which  in

5 
 RNNenc-50 
 cludes  sentences  having  un

RNNenc-30

known  words  to  the  models .

0

0   1 0   2 0   3 0   40   5 0   6 0

## Sentence  length

20 1 2  and  news-test-20 1 3  to  make  a  development  (validation)  set,  and  evaluate  the  models  on  the  test

set  (news-test-20 1 4)  from  WMT  ’ 1 4,  which  consists  of  3003  sentences  not  present  in  the  training

data .

After  a  usual  tokenization6
 ,  we  use  a  shortlist  of  30,000  most  frequent  words  in  each  language  to

train  our  models .  Any  word  not  included  in  the  shortlist  is  mapped  to  a  special  token  ( [UNK] ) .  We

do  not  apply  any  other  special  preprocessing,  such  as  lowercasing  or  stemming,  to  the  data.

4 . 2  MODELS

We  train  two  types  of  models .  The  first  one  is  an  RNN  Encoder–Decoder  (RNNencdec,  Cho  et  al. ,

20 1 4a) ,  and  the  other  is  the  proposed  model,  to  which  we  refer  as  RNNsearch.  We  train  each  model

twice:  first  with  the  sentences  of  length  up  to  30  words  (RNNencdec-30,  RNNsearch-30)  and  then

with  the  sentences  of  length  up  to  50  word  (RNNencdec-50,  RNNsearch-50) .

The  encoder  and  decoder  of  the  RNNencdec  have   1 000  hidden  units  each.7  The  encoder  of  the

RNNsearch  consists  of  forward  and  backward  recurrent  neural  networks  (RNN)  each  having   1 000

hidden  units .  Its  decoder  has   1 000  hidden  units .  In  both  cases ,  we  use  a  multilayer  network  with  a

single  maxout  (Goodfellow  et  al. ,  20 1 3)  hidden  layer  to  compute  the  conditional  probability  of  each

target  word  (Pascanu  et  al. ,  20 1 4) .

We  use  a  minibatch  stochastic  gradient  descent  (SGD)  algorithm  together  with  Adadelta  (Zeiler,

20 1 2)  to  train  each  model.  Each  SGD  update  direction  is  computed  using  a  minibatch  of  80  sen

tences .  We  trained  each  model  for  approximately  5  days .

Once  a  model  is  trained,  we  use  a  beam  search  to  find  a  translation  that  approximately  maximizes  the

conditional  probability  (see,  e. g . ,  Graves ,  20 1 2 ;  B oulanger-Lewandowski  et  al. ,  20 1 3 ) .  Sutskever

et  al.   (20 1 4)  used  this  approach  to  generate  translations  from  their  neural  machine  translation  model.

For  more  details  on  the  architectures  of  the  models  and  training  procedure  used  in  the  experiments,

see  Appendices  A  and  B .

5  RES ULTS

5 . 1  QUANTITATIVE  RESULTS

In  Table   1 ,  we  list  the  translation  performances  measured  in  BLEU  score.  It  is  clear  from  the  table

that  in  all  the  cases,  the  proposed  RNNsearch  outperforms  the  conventional  RNNencdec.  More

importantly,  the  performance  of  the  RNNsearch  is  as  high  as  that  of  the  conventional  phrase-based

translation  system  (Moses) ,  when  only  the  sentences  consisting  of  known  words  are  considered.

This  is  a  significant  achievement,  considering  that  Moses  uses  a  separate  monolingual  corpus  (4 1 8M

words)  in  addition  to  the  parallel  corpora  we  used  to  train  the  RNNsearch  and  RNNencdec .

6  We  used  the  tokenization  script  from  the  open-source  machine  translation  package,  Moses .

7
 In  this  paper,  by  a  ’ hidden  unit’ ,  we  always  mean  the  gated  hidden  unit  (see  Appendix  A. 1 . 1 ) .

5

Published  as  a  conference  paper  at  ICLR  20 1 5

tne 
 n i
c 
 t 
 ts

m ae mo d 
 ts 
 > 
 ne ne

e 
eer e 
por no ae 
sa 
 eng gu 29 
 dne dl 
 en mno
 n 
 mno >

hT ga no ht
 Eu Ec Ar w is in A
u 91 . 
 < ou dte t
 
 ir irv ts 
 ow irv dn

L' 
 It h
 s be on
 hta hte am
 ne is hte
 lae
 kn fo 
 ne . 
 <e

a c c o rd 
 I l

c o n v i e n t

s u r

d e

l a 
 n o t e r

z o n e 
 q u e

é c o n o m i q u e 
 l '

e u ro pée n n e 
 e nv i ro n n e m e nt

m a r i n

a

e s t

é t é 
 l e

s i g n é 
 m o i n s

e n 
 c o n n u

a o û t 
 d e

l '

1 9 9 2

e nv i ro n n e m e nt

. 
 .

< e n d > 
 < e n d >

( a)  (b)

ino 
 tn 
 l

tc e e 
 a 
 sn

tru imp sn 
 a 
 re 
 cu icm op d> 
 e 
 >

se f 
he qu
 ae hta 
iry na o
 gn
 dor ew he
 ae ne s l
 gn eru h
 
 liy 
 n d
 dn

D o t e m t S c n lo p n c w . 
 < ih li 
 ha y t
 it y m
 e a
 ia 
 e

La 
 " T
 w c m fu w m fa , 
 " h
t m s . 
 <

d e st ru c t i o n 
 "

d e 
 C e l a

l ' 
 v a

éq u i pe m e nt 
 c h a n g e r

s i g n i fi e 
 m o n

q u e 
 a ve n i r

l a 
 a v e c

Sy r i e 
 m a

n e 
 fa m i l l e

p e u t 
 "

p l u s

p ro d u i re 
 ,

d e 
 a

n o u ve l l e s 
 d i t

a r m e s 
 l '

c h i m i q u es 
 h o m m e

. 
 .

< e n d > 
 < e n d >

(c)  (d)

Figure  3 :  Four  sample  alignments  found  by  RNNsearch-50.  The  x-axis  and  y-axis  of  each  plot

correspond  to  the  words  in  the  source  sentence  (English)  and  the  generated  translation  (French) ,

respectively.  Each  pixel  shows  the  weight  αij  of  the  annotation  of  the  j -th  source  word  for  the  i-th

target  word  (see  Eq.  (6)) ,  in  grayscale  (0 :  black,  1 :  white) .  (a)  an  arbitrary  sentence .  (b–d)  three

randomly  selected  samples  among  the  sentences  without  any  unknown  words  and  of  length  between

1 0  and  20  words  from  the  test  set.

One  of  the  motivations  behind  the  proposed  approach  was  the  use  of  a  fixed-length  context  vector

in  the  basic  encoder–decoder  approach.  We  conj ectured  that  this  limitation  may  make  the  basic

encoder–decoder  approach  to  underperform  with  long  sentences .  In  Fig.  2,  we  see  that  the  perfor

mance  of  RNNencdec  dramatically  drops  as  the  length  of  the  sentences  increases .  On  the  other  hand,

both  RNNsearch-30  and  RNNsearch-50  are  more  robust  to  the  length  of  the  sentences .  RNNsearch

50,  especially,  shows  no  performance  deterioration  even  with  sentences  of  length  50  or  more.  This

superiority  of  the  proposed  model  over  the  basic  encoder–decoder  is  further  confirmed  by  the  fact

that  the  RNNsearch-30  even  outperforms  RNNencdec-50  (see  Table   1 ) .

6

Published  as  a  conference  paper  at  ICLR  20 1 5

Table   1 :  BLEU  scores  of  the  trained  models  com

Model  All  No  UNK◦
 puted  on  the  test  set.  The  second  and  third  columns

RNNencdec-30  1 3 . 93  24 . 1 9
 show  respectively  the  scores  on  all  the  sentences  and,

RNNsearch-30  2 1 .50  3 1 .44
 on  the  sentences  without  any  unknown  word  in  them

- selves  and  in  the  reference  translations .  Note  that

RNNencdec 50  1 7 . 82  26 .7 1 
 ?

- RNNsearch-50 was  trained  much  longer  until  the

RNNsearch 50  26.75  34. 1 6

- ?  performance  on  the  development  set  stopped  improv

RNNsearch 50 28 .45  36. 1 5

ing .  (◦)  We  disallowed  the  models  to  generate  [UNK]

Moses  3 3 . 3 0  3 5 . 63

tokens  when  only  the  sentences  having  no  unknown

words  were  evaluated  (last  column) .

5 . 2  QUALITATIVE  ANALYS IS

5 . 2 . 1  ALIGNMENT

The  proposed  approach  provides  an  intuitive  way  to  inspect  the  (soft-)alignment  between  the  words

in  a  generated  translation  and  those  in  a  source  sentence.  This  is  done  by  visualizing  the  annotation

weights  αij  from  Eq.  (6) ,  as  in  Fig .  3 .  Each  row  of  a  matrix  in  each  plot  indicates  the  weights

associated  with  the  annotations .  From  this  we  see  which  positions  in  the  source  sentence  were

considered  more  important  when  generating  the  target  word.

We  can  see  from  the  alignments  in  Fig .  3  that  the  alignment  of  words  between  English  and  French

is  largely  monotonic .  We  see  strong  weights  along  the  diagonal  of  each  matrix.  However,  we  also

observe  a  number  of  non-trivial,  non-monotonic  alignments .  Adj ectives  and  nouns  are  typically

ordered  differently  between  French  and  English,  and  we  see  an  example  in  Fig .  3  (a) .  From  this

figure,  we  see  that  the  model  correctly  translates  a  phrase  [European  Economic  Area]  into  [zone

´ ´

e conomique  europ e
en] .  The  RNNsearch  was  able  to  correctly  align  [zone]  with  [Area] , j  umping

over  the  two  words  ( [European]  and  [Economic] ) ,  and  then  looked  one  word  back  at  a  time  to

´ ´

complete  the  whole  phrase  [zone  e conomique  europ e
enne] .

The  strength  of  the  soft-alignment,  opposed  to  a  hard-alignment,  is  evident,  for  instance,  from

’

Fig .  3  (d) .  Consider  the  source  phrase  [the  man]  which  was  translated  into  [l  homme] .  Any  hard

’

alignment  will  map  [the]  to  [l ]  and  [man]  to  [homme] .  This  is  not  helpful  for  translation,  as  one

must  consider  the  word  following  [the]  to  determine  whether  it  should  be  translated  into  [le] ,  [la] ,

[les]  or  [l ’ ] .  Our  soft-alignment  solves  this  is sue  naturally  by  letting  the  model  look  at  both  [the]  and

’

[man] ,  and  in  this  example,  we  see  that  the  model  was  able  to  correctly  translate  [the]  into  [l ] .  We

observe  similar  behaviors  in  all  the  presented  cases  in  Fig .  3 .  An  additional  benefit  of  the  soft  align

ment  is  that  it  naturally  deals  with  source  and  target  phrases  of  different  lengths ,  without  requiring  a

counter-intuitive  way  of  mapping  some  words  to  or  from  nowhere  ( [NULL] )  (see,  e.g. ,  Chapters  4

and  5  of  Koehn,  20 1 0) .

5 . 2 . 2  LONG  S ENTENCES

As  clearly  visible  from  Fig .  2  the  proposed  model  (RNNsearch)  is  much  better  than  the  conventional

model  (RNNencdec)  at  translating  long  sentences .  This  is  likely  due  to  the  fact  that  the  RNNsearch

does  not  require  encoding  a  long  sentence  into  a  fixed-length  vector  perfectly,  but  only  accurately

encoding  the  parts  of  the  input  sentence  that  surround  a  particular  word.

As  an  example,  consider  this  source  sentence  from  the  test  set:

An  admitting p  rivilege  is  the  right  of  a  doctor  to  admit  a p  atient  to  a  hospital  or

a  medical  centre  to  carry  out  a  diagnosis  or  a  procedure,   based  on  his  status  as  a

health  care  worker  at  a  hospital.

The  RNNencdec-50  translated  this  sentence  into :

` ’ ’ ´ ˆ `

Un p  rivile ge  d admission  est  le  droit  d un  m e decin  de  reconna ıtre  un p  atient  a

’ ˆ ´ ’

l ho pital  ou  un  centre  m e dical  d un  diagnostic  ou  de  prendre  un  diagnostic  en

´ ´

fonction  de  son  e  tat  de  sante
.

7

Published  as  a  conference  paper  at  ICLR  20 1 5

The  RNNencdec-50  correctly  translated  the  source  sentence  until  [a  medical  center] .  However,  from

there  on  (underlined) ,  it  deviated  from  the  original  meaning  of  the  source  sentence.  For  instance,  it

replaced  [based  on  his  status  as  a  health  care  worker  at  a  hospital]  in  the  source  sentence  with  [en

´ ´ “ ”

fonction  de  son  e tat  de  sant e
 ]  ( based  on  his  state  of  health ) .

On  the  other  hand,  the  RNNsearch-50  generated  the  following  correct  translation,  preserving  the

whole  meaning  of  the  input  sentence  without  omitting  any  details :

` ’ ’ ´ ’ `

Un p  rivile ge  d admission  est  le  droit  d un  m e decin  d admettre  un p  atient  a
  un

ˆ ´ ´

ho pital  ou  un  centre  m e dical  pour  effectuer  un  diagnostic  ou  une  proce dure,   selon

´ ` ’ ˆ

son  statut  de  travailleur  des  soins  de  sante  a  l ho
pital.

Let  us  consider  another  sentence  from  the  test  set:

’ ”

This  kind  of  experience  is p  art  of D  isney s  efforts  to   extend  the  lifetime  of  its

series  and  build  new  relationships  with  audiences  via  digital  platforms  that  are

”

becoming  ever  more  important,   he  added.

The  translation  by  the  RNNencdec-50  is

’ ´ ” ´

Ce  type  d expe  riencef  ait p  artie  des  initiatives  du D  isney p  our   prolonger  la  dur e
e

´ ´

de  vie  de  ses  nouvelles  et  de  de velopper  des  liens  avec  les  lecteurs  nume  riques  qui

deviennent  plus  complexes.

As  with  the  previous  example,  the  RNNencdec  began  deviating  from  the  actual  meaning  of  the

source  sentence  after  generating  approximately  30  words  (see  the  underlined  phrase) .  After  that

point,  the  quality  of  the  translation  deteriorates ,  with  basic  mistakes  such  as  the  lack  of  a  closing

quotation  mark.

Again,  the  RNNsearch-50  was  able  to  translate  this  long  sentence  correctly :

’ ´ ” ´

Ce  genre  d expe  rience f  ait p  artie  des  efforts  de D  isney p  our   prolonger  la  dur e
e

´ ´

de  vie  de  ses  se  ries  et  cr e  er  de  nouvelles  relations  avec  des p  ublics  via  des

plateformes  nume´  riques  de  plus  en  plus  importantes ”,   a-t- il  ajoute´
.

In  conjunction  with  the  quantitative  results  presented  already,  these  qualitative  observations  con

firm  our  hypotheses  that  the  RNNsearch  architecture  enables  far  more  reliable  translation  of  long

sentences  than  the  standard  RNNencdec  model.

In  Appendix  C,  we  provide  a  few  more  sample  translations  of  long  source  sentences  generated  by

the  RNNencdec-50,  RNNsearch-50  and  Google  Translate  along  with  the  reference  translations .

6  RELATED  WORK

6 . 1  LEARNING  TO  ALIGN

A  similar  approach  of  aligning  an  output  symbol  with  an  input  symbol  was  proposed  recently  by

Graves  (20 1 3 )  in  the  context  of  handwriting  synthesis .  Handwriting  synthesis  is  a  task  where  the

model  is  asked  to  generate  handwriting  of  a  given  sequence  of  characters .  In  his  work,  he  used  a

mixture  of  Gaussian  kernels  to  compute  the  weights  of  the  annotations,  where  the  location,  width

and  mixture  coefficient  of  each  kernel  was  predicted  from  an  alignment  model.  More  specifically,

his  alignment  was  restricted  to  predict  the  location  such  that  the  location  increases  monotonically.

The  main  difference  from  our  approach  is  that,  in  (Graves ,  20 1 3 ) ,  the  modes  of  the  weights  of  the

annotations  only  move  in  one  direction.  In  the  context  of  machine  translation,  this  is  a  severe  limi

tation,  as  (long-distance)  reordering  is  often  needed  to  generate  a  grammatically  correct  translation

(for  instance,  English-to-German) .

Our  approach,  on  the  other  hand,  requires  computing  the  annotation  weight  of  every  word  in  the

source  sentence  for  each  word  in  the  translation.  This  drawback  is  not  severe  with  the  task  of

translation  in  which  most  of  input  and  output  sentences  are  only   1 5–40  words .  However,  this  may

limit  the  applicability  of  the  proposed  scheme  to  other  tasks .

8

Published  as  a  conference  paper  at  ICLR  20 1 5

6 . 2  NEURAL  NETWORKS  FOR  MACHINE  TRANSLATION

Since  B engio  et  al.   (2003)  introduced  a  neural  probabilistic  language  model  which  uses  a  neural  net

work  to  model  the  conditional  probability  of  a  word  given  a  fixed  number  of  the  preceding  words,

neural  networks  have  widely  been  used  in  machine  translation.  However,  the  role  of  neural  net

works  has  been  largely  limited  to  simply  providing  a  single  feature  to  an  existing  statistical  machine

translation  system  or  to  re-rank  a  list  of  candidate  translations  provided  by  an  existing  system.

For  instance,  Schwenk  (20 1 2)  proposed  using  a  feedforward  neural  network  to  compute  the  score  of

a  pair  of  source  and  target  phrases  and  to  use  the  score  as  an  additional  feature  in  the  phrase-based

statistical  machine  translation  system.  More  recently,  Kalchbrenner  and  Blunsom  (20 1 3)  and  Devlin

et  al.   (20 1 4)  reported  the  successful  use  of  the  neural  networks  as  a  sub-component  of  the  existing

translation  system.  Traditionally,  a  neural  network  trained  as  a  target- side  language  model  has  been

used  to  rescore  or  rerank  a  list  of  candidate  translations  (see,  e . g . ,  S chwenk  et  al. ,  2006) .

Although  the  above  approaches  were  shown  to  improve  the  translation  performance  over  the  state

of-the-art  machine  translation  systems,  we  are  more  interested  in  a  more  ambitious  obj ective  of

designing  a  completely  new  translation  system  based  on  neural  networks .  The  neural  machine  trans

lation  approach  we  consider  in  this  paper  is  therefore  a  radical  departure  from  these  earlier  works .

Rather  than  using  a  neural  network  as  a  part  of  the  existing  system,  our  model  works  on  its  own  and

generates  a  translation  from  a  source  sentence  directly.

7  CONCLUS ION

The  conventional  approach  to  neural  machine  translation,  called  an  encoder–decoder  approach,  en

codes  a  whole  input  sentence  into  a  fixed-length  vector  from  which  a  translation  will  be  decoded.

We  conj ectured  that  the  use  of  a  fixed-length  context  vector  is  problematic  for  translating  long  sen

tences ,  based  on  a  recent  empirical  study  reported  by  Cho  et  al.   (20 1 4b)  and  Pouget-Abadie  et  al.

(20 1 4) .

In  this  paper,  we  proposed  a  novel  architecture  that  addresses  this  issue.  We  extended  the  basic

encoder–decoder  by  letting  a  model  (soft-) search  for  a  set  of  input  words,  or  their  annotations  com

puted  by  an  encoder,  when  generating  each  target  word.  This  frees  the  model  from  having  to  encode

a  whole  source  sentence  into  a  fixed-length  vector,  and  also  lets  the  model  focus  only  on  information

relevant  to  the  generation  of  the  next  target  word.  This  has  a  maj or  positive  impact  on  the  ability

of  the  neural  machine  translation  system  to  yield  good  results  on  longer  sentences .  Unlike  with

the  traditional  machine  translation  systems ,  all  of  the  pieces  of  the  translation  system,  including

the  alignment  mechanism,  are j  ointly  trained  towards  a  better  log-probability  of  producing  correct

translations .

We  tested  the  proposed  model,  called  RNNsearch,  on  the  task  of  English-to-French  translation.  The

experiment  revealed  that  the  proposed  RNNsearch  outperforms  the  conventional  encoder–decoder

model  (RNNencdec)  significantly,  regardless  of  the  sentence  length  and  that  it  is  much  more  ro

bust  to  the  length  of  a  source  sentence.  From  the  qualitative  analysis  where  we  investigated  the

(soft-)alignment  generated  by  the  RNNsearch,  we  were  able  to  conclude  that  the  model  can  cor

rectly  align  each  target  word  with  the  relevant  words ,  or  their  annotations ,  in  the  source  sentence  as

it  generated  a  correct  translation.

Perhaps  more  importantly,  the  proposed  approach  achieved  a  translation  performance  comparable  to

the  existing  phrase-based  statistical  machine  translation.  It  is  a  striking  result,  considering  that  the

proposed  architecture,  or  the  whole  family  of  neural  machine  translation,  has  only  been  proposed

as  recently  as  this  year.  We  believe  the  architecture  proposed  here  is  a  promising  step  toward  better

machine  translation  and  a  better  understanding  of  natural  languages  in  general.

One  of  challenges  left  for  the  future  is  to  better  handle  unknown,  or  rare  words .  This  will  be  required

for  the  model  to  be  more  widely  used  and  to  match  the  performance  of  current  state-of-the-art

machine  translation  systems  in  all  contexts .

9

Published  as  a  conference  paper  at  ICLR  20 1 5

ACKNOWLEDGMENTS

The  authors  would  like  to  thank  the  developers  of  Theano  (B ergstra  et  al. ,  20 1 0 ;  B astien  et  al. ,

20 1 2) .  We  acknowledge  the  support  of  the  following  agencies  for  research  funding  and  computing

support:  NSERC,  Calcul  Que´
bec,  Compute  Canada,  the  Canada  Research  Chairs  and  CIFAR.  B ah-

danau  thanks  the  support  from  Planet  Intelligent  Systems  GmbH.  We  also  thank  Felix  Hill,  B art  van

Merrie´
nboer,  Jean  Pouget-Abadie,  Coline  Devin  and  Tae-Ho  Kim.

REFERENCES

Axelrod,  A. ,  He,  X. ,  and  Gao,  J.  (20 1 1 ) .  Domain  adaptation  via  pseudo  in-domain  data  selection.

In  Proceedings  of  the A  CL  Conference  on  Empirical M  ethods  in N  atural L  anguage  Processing

(EMNLP),  pages  355–362.  Association  for  Computational  Linguistics .

B astien,  F. ,  Lamblin,  P. ,  Pascanu,  R. ,  B ergstra,  J. ,  Goodfellow,  I.  J. ,  B ergeron,  A. ,  B ouchard,  N. ,

and  Bengio,  Y.  (20 1 2) .  Theano :  new  features  and  speed  improvements .  Deep  Learning  and

Unsupervised  Feature  Learning  NIPS  20 1 2  Workshop.

B engio,  Y. ,  Simard,  P. ,  and  Frasconi,  P.  ( 1 994) .  Learning  long-term  dependencies  with  gradient

descent  is  difficult.  IEEE  Transactions  on N  eural N  etworks,  5(2) ,   1 57– 1 66 .

B engio,  Y. ,  Ducharme,  R. ,  Vincent,  P. ,  and  Janvin,  C .  (2003 ) .  A  neural  probabilistic  language  model.

J. M  ach. L  earn. R  es. ,  3 ,   1 1 3 7– 1 1 5 5 .

B ergstra,  J. ,  Breuleux,  O . ,  B astien,  F. ,  Lamblin,  P. ,  Pascanu,  R. ,  Desj ardins ,  G. ,  Turian,  J. ,  Warde

Farley,  D . ,  and  B engio,  Y.  (20 1 0) .  Theano :  a  CPU  and  GPU  math  expression  compiler.  In

Proceedings  of the  Python f  or  Scientific  Computing  Conference  (SciPy) .  Oral  Presentation.

B oulanger-Lewandowski,  N. ,  Bengio,  Y. ,  and  Vincent,  P.  (20 1 3) .  Audio  chord  recognition  with

recurrent  neural  networks .  In  ISMIR.

Cho,  K. ,  van  Merrienboer,  B . ,  Gulcehre,  C . ,  B ougares ,  F. ,  Schwenk,  H. ,  and  B engio,  Y.  (20 1 4a) .

Learning  phrase  representations  using  RNN  encoder-decoder  for  statistical  machine  translation.

In  Proceedings  of  the  Empiricial M  ethods  in N  atural L  anguage  Processing  (EMNLP  2014) .  to

appear.

¨

Cho,  K. ,  van  Merrie
nboer,  B . ,  B ahdanau,  D . ,  and  B engio,  Y.  (20 1 4b) .  On  the  properties  of  neural

machine  translation:  Encoder–Decoder  approaches .  In  Eighth  Workshop  on  Syntax,  Semantics

and  Structure  in  Statistical  Translation .  to  appear.

Devlin,  J. ,  Zbib,  R. ,  Huang,  Z. ,  Lamar,  T. ,  Schwartz,  R. ,  and  Makhoul,  J.  (20 1 4) .  Fast  and  robust

neural  network j  oint  models  for  statistical  machine  translation.  In  Association f  or  Computational

Linguistics .

˜

Forcada,  M .  L.  and  N
 eco,  R.  P.  ( 1 997) .  Recursive  hetero-associative  memories  for  translation.  In

J.  Mira,  R.  Moreno-Dı´az,  and  J.  Cabestany,  editors,  Biological  and A  rtificial  Computation:  From

Neuroscience  to  Technology,  volume   1 240  of  Lecture N  otes  in  Computer  Science,  pages  45 3–462.

Springer  B erlin  Heidelberg .

Goodfellow,  I. ,  Warde-Farley,  D . ,  Mirza,  M. ,  Courville,  A. ,  and  B engio,  Y.  (20 1 3) .  Maxout  net

works .  In  Proceedings  of  The  30th I  nternational  Conference  on M  achine L  earning,  pages   1 3 1 9–

1 3 27 .

Graves,  A.  (20 1 2) .  Sequence  transduction  with  recurrent  neural  networks .  In  Proceedings  of  the

29th I  nternational  Conference  on M  achine L  earning  (ICML  2012) .

Graves,  A.  (20 1 3) .  Generating  sequences  with  recurrent  neural  networks .  arXiv: 1 3 0 8 . 0 8 5 0

[ c s . N E ] .

Graves,  A. ,  Jaitly,  N. ,  and  Mohamed,  A. -R.  (20 1 3) .  Hybrid  speech  recognition  with  deep  bidirec

tional  LSTM.  In  Automatic  Speech R  ecognition  and  Understanding  (ASRU),  2013 I  EEE  Work

shop  on,  pages  273–27 8 .

1 0

Published  as  a  conference  paper  at  ICLR  20 1 5

Hermann,  K.  and  Blunsom,  P.  (20 1 4) .  Multilingual  distributed  representations  without  word  align

ment.  In  Proceedings  of the  Second I  nternational  Conference  on L  earning R  epresentations  (ICLR

2 01 4) .

Hochreiter,  S .  ( 1 99 1 ) .  Untersuchungen  zu  dynamischen  neuronalen  Netzen.  Diploma  thesis,  Institut

¨ ¨ ¨

fu r  Informatik,  Lehrstuhl  Prof.  Brauer,  Technische  Universit a t  Mu
 nchen.

Hochreiter,  S .  and  Schmidhuber,  J.  ( 1 997) .  Long  short-term  memory.  Neural  Computation,  9(8) ,

1 7 3 5– 1 7 80 .

Kalchbrenner,  N.  and  Blunsom,  P.  (20 1 3) .  Recurrent  continuous  translation  models .  In  Proceedings

of  the A  CL  Conference  on  Empirical M  ethods  in N  atural L  anguage  Processing  (EMNLP),  pages

1 700– 1 709 .  Association  for  Computational  Linguistics .

Koehn,  P.  (20 1 0) .  Statistical M  achine  Translation.  Cambridge  University  Press,  New  York,  NY,

USA.

Koehn,  P. ,  Och,  F.  J. ,  and  Marcu,  D .  (2003 ) .  Statistical  phrase-based  translation.  In  Proceedings

of  the  2003  Conference  of  the N  orth A  merican  Chapter  of  the A  ssociation f  or  Computational

Linguistics  on H  uman L  anguage  Technology  -  Volume  1 ,  NAACL  ’ 03 ,  pages  48–54,  Stroudsburg,

PA,  USA.  Association  for  Computational  Linguistics .

Pascanu,  R. ,  Mikolov,  T. ,  and  B engio,  Y.  (20 1 3 a) .  On  the  difficulty  of  training  recurrent  neural

’

networks .  In  ICML 2013.

Pascanu,  R. ,  Mikolov,  T. ,  and  B engio,  Y.  (20 1 3b) .  On  the  difficulty  of  training  recurrent  neural

networks .  In  Proceedings  of  the  30th I  nternational  Conference  on M  achine L  earning  (ICML

2 01 3) .

Pascanu,  R. ,  Gulcehre,  C . ,  Cho,  K. ,  and  B engio,  Y.  (20 1 4) .  How  to  construct  deep  recurrent  neural

networks .  In  Proceedings  of  the  Second I  nternational  Conference  on L  earning R  epresentations

(ICLR  2014) .

Pouget-Abadie,  J. ,  B ahdanau,  D . ,  van  Merrie¨
nboer,  B . ,  Cho,  K. ,  and  B engio,  Y.  (20 1 4) .  Overcoming

the  curse  of  sentence  length  for  neural  machine  translation  using  automatic  segmentation.  In

Eighth  Workshop  on  Syntax,  Semantics  and  Structure  in  Statistical  Translation.  to  appear.

Schuster,  M.  and  Paliwal,  K.  K.  ( 1 997) .  Bidirectional  recurrent  neural  networks .  Signal  Processing,

IEEE  Transactions  on,  45( 1 1 ) ,  2673–268 1 .

Schwenk,  H.  (20 1 2) .  Continuous  space  translation  models  for  phrase-based  statistical  machine

translation.  In  M.  Kay  and  C .  B oitet,  editors,  Proceedings  of the  24th I  nternational  Conference  on

Computational L  inguistics  (COLIN),  pages   1 07 1 – 1 080.  Indian  Institute  of  Technology  Bombay.

Schwenk,  H. ,  Dchelotte,  D . ,  and  Gauvain,  J. -L.  (2006) .  Continuous  space  language  models  for

statistical  machine  translation.  In  Proceedings  of  the  COLING/ACL  on M  ain  conference p  oster

sessions,  pages  723–730.  Association  for  Computational  Linguistics .

Sutskever,  I. ,  Vinyals ,  O . ,  and  Le,  Q.  (20 1 4) .  Sequence  to  sequence  learning  with  neural  networks .

In  Advances  in N  eural I  nformation  Processing  Systems  (NIPS  2014) .

Zeiler,  M.  D .  (20 1 2) .  ADADELTA:  An  adaptive  learning  rate  method.  arXiv: 1 2 1 2 . 5 7 0 1

[ c s . L G ] .

1 1

Published  as  a  conference  paper  at  ICLR  20 1 5

A  MODEL  ARCHITECTURE

A . 1  ARCHITECTURAL  CHOICES

The  proposed  scheme  in  Section  3  is  a  general  framework  where  one  can  freely  define,  for  instance,

the  activation  functions  f  of  recurrent  neural  networks  (RNN)  and  the  alignment  model  a .  Here,  we

describe  the  choices  we  made  for  the  experiments  in  this  paper.

A . 1 . 1  RECURRENT  NEURAL  NETWORK

For  the  activation  function  f  of  an  RNN,  we  use  the  gated  hidden  unit  recently  proposed  by  Cho

et  al.   (20 1 4a) .  The  gated  hidden  unit  is  an  alternative  to  the  conventional  simple  units  such  as  an

element-wise  tanh.  This  gated  unit  is  similar  to  a  long  short-term  memory  (LSTM)  unit  proposed

earlier  by  Hochreiter  and  Schmidhuber  ( 1 997) ,  sharing  with  it  the  ability  to  better  model  and  learn

long-term  dependencies .  This  is  made  possible  by  having  computation  paths  in  the  unfolded  RNN

for  which  the  product  of  derivatives  is  close  to   1 .  These  paths  allow  gradients  to  flow  backward

easily  without  suffering  too  much  from  the  vanishing  effect  (Hochreiter,   1 99 1 ;  B engio  et  al. ,   1 994 ;

Pascanu  et  al. ,  20 1 3 a) .  It  is  therefore  pos sible  to  use  LSTM  units  instead  of  the  gated  hidden  unit

described  here,  as  was  done  in  a  similar  context  by  Sutskever  et  al.   (20 1 4) .

The  new  state  si  of  the  RNN  employing  n  gated  hidden  units8
 is  computed  by

s i  =  f ( s i − 1 ,  yi − 1 ,  ci )  =   ( 1   −   zi )   ◦  s i − 1  +  zi  ◦  s˜i
 ,

where  ◦  is  an  element-wise  multiplication,  and  zi
 is  the  output  of  the  update  gates  (see  below) .  The

˜

proposed  updated  state  si
 is  computed  by

˜

si  =  t anh   ( W e ( yi − 1 )  +  U  [ri  ◦  si − 1 ]  +  Cci ) ,

where  e (yi − 1 )   ∈  R
m  is  an  m-dimensional  embedding  of  a  word  yi − 1 ,  and  ri
 is  the  output  of  the

reset  gates  (see  below) .  When  yi
 is  represented  as  a  1 -of-K  vector,  e (yi )   is  simply  a  column  of  an

embedding  matrix  E  ∈  R
m × K .  Whenever  possible,  we  omit  bias  terms  to  make  the  equations  less

cluttered.

The  update  gates  zi  allow  each  hidden  unit  to  maintain  its  previous  activation,  and  the  reset  gates  ri

control  how  much  and  what  information  from  the  previous  state  should  be  reset.  We  compute  them

by

zi  =  σ  ( Wz e (yi − 1 )  +  Uz si − 1  +  Cz ci ) ,

ri  =  σ  ( Wr e (yi − 1 )  +  Ur si − 1  +  Cr ci ) ,

where  σ  ( · )   is  a  logistic  sigmoid  function.

At  each  step  of  the  decoder,  we  compute  the  output  probability  (Eq.  (4))  as  a  multi-layered  func

tion  (Pascanu  et  al. ,  20 1 4) .  We  use  a  single  hidden  layer  of  maxout  units  (Goodfellow  et  al. ,  20 1 3 )

and  normalize  the  output  probabilities  (one  for  each  word)  with  a  softmax  function  (see  Eq.  (6)) .

A . 1 . 2  ALIGNMENT  MODEL

The  alignment  model  should  be  designed  considering  that  the  model  needs  to  be  evaluated  Tx  ×   Ty

times  for  each  sentence  pair  of  lengths  Tx  and  Ty .  In  order  to  reduce  computation,  we  use  a  single

layer  multilayer  perceptron  such  that

a ( s i − 1 ,  hj  )  =  v
a>
 
 t anh   ( Wa s i − 1  +  Ua hj  ) ,

where  Wa  ∈  R
n × n ,  Ua  ∈  R
n × 2n  and  va  ∈  R
n  are  the  weight  matrices .  Since  Ua hj  does  not

depend  on  i ,  we  can  pre-compute  it  in  advance  to  minimize  the  computational  cost.

8  Here,  we  show  the  formula  of  the  decoder.  The  same  formula  can  be  used  in  the  encoder  by  simply

ignoring  the  context  vector  ci  and  the  related  terms .

1 2

Published  as  a  conference  paper  at  ICLR  20 1 5

A . 2  DETAILED  DES CRIPTION  OF  THE  MODEL

A . 2 . 1  ENCODER

In  this  section,  we  describe  in  detail  the  architecture  of  the  proposed  model  (RNNsearch)  used  in  the

experiments  (see  Sec .  4–5) .  From  here  on,  we  omit  all  bias  terms  in  order  to  increase  readability.

The  model  takes  a  source  sentence  of   1 -of-K  coded  word  vectors  as  input

x  =   ( x 1 ,   .   .   .   ,  x Tx
 ) ,  x i  ∈   R
Kx

and  outputs  a  translated  sentence  of   1 -of-K  coded  word  vectors

y  =   ( y 1 ,   .   .   .   ,  yTy
 ) ,  yi  ∈   R
Ky
 ,

where  Kx  and  Ky  are  the  vocabulary  sizes  of  source  and  target  languages,  respectively.  Tx  and  Ty

respectively  denote  the  lengths  of  source  and  target  sentences .

First,  the  forward  states  of  the  bidirectional  recurrent  neural  network  (BiRNN)  are  computed:

− →− − →−

→− ( 1   − 
 →z  i )   ◦
 h  i − 1  +
 →z  i  ◦
 h  i
 ,  if  i   >   0

h  i  =

(

0  ,  if  i  =  0

where

→− −→ →− 
− →−

h  i  =  t anh  WExi  +
 U
 →r  i  ◦
 h  i − 1

 h i 

− −→ →− →−

→z  i  = σ
 Wz Exi  +
 U  z
 h  i − 1

 

− −→ →− →−

→r  i  = σ
 Wr Exi  +
 U  r
 h  i − 1
 .

 

−→ −→ −→ →− →− →−

E  ∈  R
m × Kx  is  the  word  embedding  matrix .
 W  ,
 Wz ,
 Wr  ∈  R
n × m ,
 U   ,
 U  z ,
 U  r  ∈  R
n × n  are

weight  matrices .  m  and  n  are  the  word  embedding  dimensionality  and  the  number  of  hidden  units,

respectively.  σ ( · )   is  as  usual  a  logistic  sigmoid  function.

←−
 ←−

The  backward  states  (
 h  1 ,  ·  ·  ·  ,
 h  Tx
 )   are  computed  similarly.  We  share  the  word  embedding  matrix

E  between  the  forward  and  backward  RNNs,  unlike  the  weight  matrices .

We  concatenate  the  forward  and  backward  states  to  to  obtain  the  annotations  ( h 1 ,  h2 ,  ·  ·  ·  ,  hTx
 ) ,

where

→−

hi  =
 ←h−  
i
 (7 )

" h  i
 #

A . 2 . 2  DECODER

The  hidden  state  si  of  the  decoder  given  the  annotations  from  the  encoder  is  computed  by

s i  = ( 1   −   zi )   ◦  s i − 1   +  zi  ◦  s˜i
 ,

where

˜

si  =  t anh   ( W Eyi − 1  +  U  [ri  ◦  si − 1 ]  +  Cci )

zi  =σ  ( Wz Eyi − 1  +  Uz si − 1  +  Cz ci )

ri  =σ  ( Wr Eyi − 1  +  Ur si − 1  +  Cr ci )

E  is  the  word  embedding  matrix  for  the  target  language.  W,  Wz ,  Wr  ∈  R
n × m ,  U,  Uz ,  Ur  ∈  R
n × n ,

and  C,  Cz ,  Cr  ∈  R
n × 2n  are  weights .  Again,  m  and  n  are  the  word  embedding  dimensionality

and  the
  number
  of  hidden  units ,  respectively.  The  initial  hidden  state  s0  is  computed  by  s0  =

←−
 n × n

 

t anh  Ws
 h  1
 ,  where  Ws  ∈   R
 .

The  context  vector  ci  are  recomputed  at  each  step  by  the  alignment  model :

Tx

ci  =
 X αij hj  ,

j = 1

1 3

Published  as  a  conference  paper  at  ICLR  20 1 5

Model  Updates  ( × 105
)  Epochs  Hours  GPU  Train  NLL  Dev.  NLL

RNNenc-30  8 .46  6.4   1 09  TITAN  BLACK  28 . 1  5 3 .0

RNNenc-50  6 .00  4. 5   1 08  Quadro  K-6000  44.0  43 . 6

RNNsearch-30  4.7 1  3 .6   1 1 3  TITAN  BLACK  26.7  47 .2

RNNsearch-50  2. 8 8  2. 2   1 1 1   Quadro  K-6000  40.7  3 8 . 1

RNNsearch-50?  6 . 67  5 .0  252  Quadro  K-6000  3 6 .7  3 5 . 2

# Table  2 :  Learning  statistics  and  relevant  information.  Each  update  corresponds  to  updating  the

# parameters  once  using  a  single  minibatch.  One  epoch  is  one  pass  through  the  training  set.  NLL  is

# the  average  conditional  log-probabilities  of  the  sentences  in  either  the  training  set  or  the  development

set.  Note  that  the  lengths  of  the  sentences  differ.

where

exp   ( eij  )

α ij  =
 Tx

P k = 1  exp   ( eik )

eij  = v
a>
 
 t anh   ( Wa si − 1  +  Ua hj  ) ,

- n
0
 n
0 × n

and  hj  is  the  j th  annotation  in  the  source  sentence  (see  Eq.  (7)) .  va  ∈  R
 ,  Wa  ∈  R
 and

Ua  ∈  R
n
0 × 2n  are  weight  matrices .  Note  that  the  model  becomes  RNN  Encoder–Decoder  (Cho

# →−

e t  al. ,  20 1 4a) ,  if  we  fix  ci
 to
 h  Tx
 .

With  the  decoder  state  si − 1 ,  the  context  ci  and  the  last  generated  word  yi − 1 ,  we  define  the  probability

of  a  target  word  yi  as

where

>

p ( yi
 | s i
 ,  yi − 1 ,  ci )   ∝  exp    y
i  Wo ti
  ,

˜ ˜ >

ti  =
 max   ti , 2j − 1 , ti , 2j
 	  j = 1 , . . . , l

# ˜ ˜

and  ti , k  is  the  k-th  element  of  a  vector  ti  which  is  computed  by

# ˜

ti  = Uo si − 1  +  Vo Eyi − 1  +  Co ci
 .

Wo  ∈  R
Ky × l
 ,  Uo  ∈  R
2 l × n ,  Vo  ∈  R
2 l × m  and  Co  ∈  R
2 l × 2n  are  weight  matrices .  This  can  be  under

stood  as  having  a  deep  output  (Pascanu  et  al. ,  20 1 4)  with  a  single  maxout  hidden  layer  (Goodfellow

e t  al. ,  20 1 3 ) .

A . 2 . 3  MODEL  S IZE

For  all  the  models  used  in  this  paper,  the  size  of  a  hidden  layer  n  is   1 000,  the  word  embedding

dimensionality  m  is  620  and  the  size  of  the  maxout  hidden  layer  in  the  deep  output  l  is  500 .  The

number  of  hidden  units  in  the  alignment  model  n
0
 is   1 000 .

B  TRAINING  PROCEDURE

B . 1  PARAMETER  INITIALIZATION

←−
 ←−
 ←−
 →− →− →−

We  initialized  the  recurrent  weight  matrices  U,  Uz ,  Ur ,
 U   ,
 U  z ,
 U  r ,
 U   ,
 U  z  and
 U  r  as  random  or

thogonal  matrices .  For  Wa  and  Ua ,  we  initialized  them  by  sampling  each  element  from  the  Gaussian

distribution  of  mean  0  and  variance  0 . 00 1 2
 .  All  the  elements  of  Va  and  all  the  bias  vectors  were  ini

# tialized  to  zero .  Any  other  weight  matrix  was  initialized  by  sampling  from  the  Gaussian  distribution

of  mean  0  and  variance  0 . 0 1 2
 .

B . 2  TRAINING

# We  used  the  stochastic  gradient  descent  (SGD)  algorithm.  Adadelta  (Zeiler,  20 1 2)  was  used  to

automatically  adapt  the  learning  rate  of  each  parameter  (  =  1 0 − 6
 and  ρ  =  0 . 95) .  We  explicitly

# 1 4

Published  as  a  conference  paper  at  ICLR  20 1 5

normalized  the  L2 -norm  of  the  gradient  of  the  cost  function  each  time  to  be  at  most  a  predefined

threshold  of  1 ,  when  the  norm  was  larger  than  the  threshold  (Pascanu  et  al. ,  20 1 3b) .  Each  S GD

update  direction  was  computed  with  a  minibatch  of  80  sentences .

At  each  update  our  implementation  requires  time  proportional  to  the  length  of  the  longest  sentence  in

a  minibatch.  Hence,  to  minimize  the  waste  of  computation,  before  every  20-th  update,  we  retrieved

1 600  sentence  pairs ,  sorted  them  according  to  the  lengths  and  split  them  into  20  minibatches .  The

training  data  was  shuffled  once  before  training  and  was  traversed  sequentially  in  this  manner.

In  Tables  2  we  present  the  statistics  related  to  training  all  the  models  used  in  the  experiments .

C  TRANSLATIONS  OF  LONG  S ENTENCES

S ource  An  admitting  privilege  is  the  right  of  a  doctor  to  admit  a  patient  to  a  hospital  or  a  medical  centre

to  carry  out  a  diagnosis  or  a  procedure,  based  on  his  status  as  a  health  care  worker  at  a  hospital .

` ’ ’ ´

Reference  Le  privile ge  d admission  est  le  droit  d un  m e
decin,  en  vertu  de  son  statut  de  membre  soignant

’ ˆ ’ ˆ ´ ’ ´

d un  ho pital,  d admettre  un  patient  dans  un  h o pital  ou  un  centre  m e dical  afin  d y  d e
livrer  un

diagnostic  ou  un  traitement.

- ` ’ ’ ´ ˆ ` ’ ˆ

RNNenc 50  Un  privile ge  d admission  est  le  droit  d un  m e decin  de  reconna ıtre  un  patient  a   l h o
pital  ou  un

´ ’ ´ ´

centre  me dical  d un  diagnostic  ou  de  prendre  un  diagnostic  en  fonction  de  son  e tat  de  sant e
 .

- ` ’ ’ ´ ’ ` ˆ

RNNsearch 50  Un  privile ge  d admission  est  le  droit  d un  m e decin  d admettre  un  patient  a   un  h o
pital  ou  un

´ ´

centre  me dical  pour  effectuer  un  diagnostic  ou  une  proc e
dure,  selon  son  statut  de  travailleur  des

´ ` ’ ˆ

soins  de  s ante  a   l h o
 pital .

` ’ ´ ’ ˆ

Google
 Un  privile ge  admettre  est  le  droit  d un  m e decin  d admettre  un  patient  dans  un  h o
pital  ou  un

´ ´ ´

Translate
 centre  me dical  pour  effectuer  un  diagnostic  ou  une  proc e dure,  fond e
e  sur  sa  situation  en  tant

´ ˆ

que  travailleur  de  soins  de  sante   dans  un  h o
 pital .

’ ”

S ource  This  kind  of  experience  is  part  of  Disney s  efforts  to   extend  the  lifetime  of  its  series  and  build

”

new  relationships  with  audiences  via  digital  platforms  that  are  becoming  ever  more  important,

he  added.

’ ´ ” ´ ´

Reference  Ce  type  d expe rience  entre  dans  le  cadre  des  efforts  de  Disney  pour    e tendre  la  dur e
e  de

´ ˆ `

vie  de  ses  se ries  et  construire  de  nouvelles  relations  avec  son  public  gr a ce  a
  des  plateformes

nume´ riques  qui  sont  de  plus  en  plus  importantes” ,  a-t-il  aj out e´
 .

- ’ ´ ” ´

RNNenc 50  Ce  type  d expe rience  fait  partie  des  initiatives  du  Disney  pour   prolonger  la  dur e
e  de  vie  de

´ ´ -

ses  nouvelles  et  de  de velopper  des  liens  avec  les  lecteurs  num e
riques  qui  deviennent  plus  com

plexes .

- ’ ´ ” ´

RNNsearch 50  Ce  genre  d expe rience  fait  partie  des  efforts  de  Disney  pour   prolonger  la  dur e
e  de  vie  de  ses

´ ´ ´

se ries  et  cr e er  de  nouvelles  relations  avec  des  publics  via  des  plateformes  num e
riques  de  plus

en  plus  importantes” ,  a-t-il  aj oute´
 .

’ ´ ` “ ´ ´ ´

Google
 Ce  genre  d expe rience  fait  partie  des  efforts  de  Disney  a     e tendre  la  dur e e  de  vie  de  sa  s e
rie  et

Translate
 construire  de  nouvelles  relations  avec  le  public  par  le  biais  des  plates-formes  nume´
riques  qui

deviennent  de  plus  en  plus  important” ,  at-il  aj oute´
 .

Source  In  a  press  conference  on  Thursday,  Mr  Blair  stated  that  there  was  nothing  in  this  video  that  might

” ”

constitute  a   reasonable  motive  that  could  lead  to  criminal  charges  being  brought  against  the

mayor.

´ ´ ’ ’ ´

Reference  En  confe rence  de  pres se, j  eudi,  M .  B lair  a  affirm e   qu il  n y  avait  rien  dans  cette  vid e
o  qui  puis se

” ” ´ ˆ ’

constituer  des   motifs  raisonnables  pouvant  mener  au  de po
t  d une  accusation  criminelle  contre

le  maire .

- ´ ’ ’ ´

RNNenc 50  Lors  de  la  confe rence  de  pres se  de j  eudi,  M .  B lair  a  dit  qu il  n y  avait  rien  dans  cette  vid e
o  qui

” ” ˆ

pourrait  constituer  une   motivation  raisonnable  pouvant  entraıner  des  accusations  criminelles

´

porte
es  contre  le  maire .

- ’ ´ ´ ´ ’ ’ ´

RNNsearch 50  Lors  d une  confe rence  de  pres se j  eudi,  M .  B lair  a  d e clar e   qu il  n y  avait  rien  dans  cette  vid e
o  qui

” ” `

pourrait  constituer  un   motif  raisonnable  qui  pourrait  conduire  a
  des  accusations  criminelles

contre  le  maire .

’ ´ ´ ´ ’ ’

Google
 Lors  d une  confe rence  de  pres se j  eudi,  M .  B lair  a  d e clar e
  qu il  n y  avait  rien  dans  cette  vido

” ” `

Translate
 qui  pourrait  constituer  un   motif  raisonnable  qui  pourrait  mener  a
  des  accusations  criminelles

portes  contre  le  maire .

Table  3 :  The  translations  generated  by  RNNenc-50  and  RNNsearch-50  from  long  source  sentences

###### (30  words  or  more)  selected  from  the  test  set.  For  each  source  sentence,  we  also  show  the  gold

standard  translation.  The  translations  by  Google  Translate  were  made  on  27  August  20 1 4.

###### 1 5
