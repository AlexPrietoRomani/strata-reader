## BERT :  Pre-training  of  Deep  Bidirectional  Transformers  for

## Language  Understanding

###### Jacob  Devlin  Ming-Wei  Chang  Kenton  Lee  Kristina  Toutanova

Google  AI  Language

{ j a c obdevl i n , mi ngwe i chang , ke nt on l , k r i s t out } @ go o g l e . c om

Abstract
 There  are  two  existing  strategies  for  apply

 ing  pre-trained  language  representations  to  down

We  introduce  a  new  language  representa

stream  tasks :  feature-based  and  fine-tuning.  The

9 tion  model  called  BERT,  which  stands  for

1 Bidirectional  Encoder  Representations  from
 feature-based  approach,  such  as  ELMo  (Peters

0 Transformers .  Unlike  recent  language  repre et  al. ,  20 1 8a) ,  uses  task- specific  architectures  that

2  sentation  models  (Peters  et  al. ,  20 1 8a;  Rad include  the  pre-trained  representations  as  addi

y ford  et  al. ,  20 1 8) ,  BERT  is  designed  to  pre tional  features .  The  fine-tuning  approach,  such  as

# a

train  deep  bidirectional  representations  from
 the  Generative  Pre-trained  Transformer  (OpenAI

M unlabeled  text  by j  ointly  conditioning  on  both

GPT)  (Radford  et  al. ,  20 1 8) ,  introduces  minimal

4 left  and  right  context  in  all  layers .  As  a  re -

-  task specific  parameters ,  and  is  trained  on  the

2 sult,  the  pre trained  BERT  model  can  be  fine

tuned  with j  ust  one  additional  output  layer
 downstream  tasks  by  simply  fine-tuning  all  pre

] to  create  state-of-the-art  models  for  a  wide
 trained  parameters .  The  two  approaches  share  the

L range  of  tasks,  such  as  question  answering  and
 same  obj ective  function  during  pre-training,  where

C. language  inference,  without  substantial  task they  use  unidirectional  language  models  to  learn

s specific  architecture  modifications .
 general  language  representations .

# c

# [

BERT  is  conceptually  simple  and  empirically
 We  argue  that  current  techniques  restrict  the

2  powerful.  It  obtains  new  state-of-the-art  re power  of  the  pre-trained  representations ,  espe

v sults  on  eleven  natural  language  processing
 cially  for  the  fine-tuning  approaches .  The  ma

5 tasks,  including  pushing  the  GLUE  score  to

j or  limitation  is  that  standard  language  models  are

0 80.5 %  (7 .7 %  point  absolute  improvement),

8 unidirectional,  and  this  limits  the  choice  of  archi

MultiNLI  accuracy  to  86 .7 %  (4. 6%  absolute

4 improvement),  SQuAD  v 1 . 1  question  answer tectures  that  can  be  used  during  pre-training.  For

0. ing  Test  F 1  to  93 .2  ( 1 .5  point  absolute  im example,  in  OpenAI  GPT,  the  authors  use  a  left-to

01 provement)  and  SQuAD  v2.0  Test  F 1  to  83 . 1
 right  architecture,  where  every  token  can  only  at

8 (5 . 1  point  absolute  improvement) .
 tend  to  previous  tokens  in  the  self-attention  layers

1: of  the  Transformer  (Vaswani  et  al. ,  20 1 7) .  Such  re

1  Introduction
 - -

v strictions  are  sub optimal  for  sentence level  tasks ,

# i

Language  model  pre-training  has  been  shown  to
 and  could  be  very  harmful  when  applying  fine

Xr be  effective  for  improving  many  natural  language
 tuning  based  approaches  to  token-level  tasks  such

a proces sing  tasks  (Dai  and  Le,  20 1 5 ;  Peters  et  al. ,
 as  question  answering,  where  it  is  crucial  to  incor

20 1 8a;  Radford  et  al. ,  20 1 8 ;  Howard  and  Ruder,
 porate  context  from  both  directions .

20 1 8) .  These  include  sentence-level  tasks  such  as
 In  this  paper,  we  improve  the  fine-tuning  based

natural  language  inference  (B owman  et  al. ,  20 1 5 ;
 approaches  by  proposing  BERT:  Bidirectional

Williams  et  al. ,  20 1 8)  and  paraphrasing  (Dolan
 Encoder  Representations  from  Transformers .

and  Brockett,  2005) ,  which  aim  to  predict  the  re BERT  alleviates  the  previously  mentioned  unidi

lationships  between  sentences  by  analyzing  them
 rectionality  constraint  by  using  a  “masked  lan

holistically,  as  well  as  token-level  tasks  such  as
 guage  model”  (MLM)  pre-training  obj ective,  in

named  entity  recognition  and  question  answering,
 spired  by  the  Cloze  task  (Taylor,  1 95 3 ) .  The

where  models  are  required  to  produce  fine-grained
 masked  language  model  randomly  masks  some  of

output  at  the  token  level  (Tj ong  Kim  S ang  and
 the  tokens  from  the  input,  and  the  obj ective  is  to

De  Meulder,  2003 ;  Rajpurkar  et  al. ,  20 1 6) .
 predict  the  original  vocabulary  id  of  the  masked

word  based  only  on  its  context.  Unlike  left-to These  approaches  have  been  generalized  to

right  language  model  pre-training,  the  MLM  ob coarser  granularities,  such  as  sentence  embed

j ective  enables  the  representation  to  fuse  the  left
 dings  (Kiros  et  al. ,  20 1 5 ;  Logeswaran  and  Lee,

and  the  right  context,  which  allows  us  to  pre 20 1 8)  or  paragraph  embeddings  (Le  and  Mikolov,

train  a  deep  bidirectional  Transformer.  In  addi 20 1 4) .  To  train  sentence  representations ,  prior

tion  to  the  masked  language  model,  we  also  use
 work  has  used  obj ectives  to  rank  candidate  next

a  “next  sentence  prediction”  task  that j  ointly  pre sentences  (Jernite  et  al. ,  20 1 7 ;  Logeswaran  and

trains  text-pair  representations .  The  contributions
 Lee,  20 1 8) ,  left-to-right  generation  of  next  sen

of  our  paper  are  as  follows :
 tence  words  given  a  representation  of  the  previous

sentence  (Kiros  et  al. ,  20 1 5) ,  or  denoising  auto

•  We  demonstrate  the  importance  of  bidirectional

encoder  derived  obj ectives  (Hill  et  al . ,  20 1 6) .

pre-training  for  language  representations .  Un

 ELMo  and  its  predeces sor  (Peters  et  al . ,  20 1 7 ,

like  Radford  et  al.  (20 1 8) ,  which  uses  unidirec

- 20 1 8a)  generalize  traditional  word  embedding  re

tional  language  models  for  pre training,  BERT

 search  along  a  different  dimension.  They  extract

uses  masked  language  models  to  enable  pre

context-sensitive  features  from  a  left-to-right  and  a

trained  deep  bidirectional  representations .  This

right-to-left  language  model.  The  contextual  rep

is  also  in  contrast  to  Peters  et  al .  (20 1 8 a) ,  which

resentation  of  each  token  is  the  concatenation  of

uses  a  shallow  concatenation  of  independently

- - - - the  left-to-right  and  right-to-left  representations .

trained  left to right  and  right to left  LMs .

When  integrating  contextual  word  embeddings

•  We  show  that  pre-trained  representations  reduce
 with  existing  task-specific  architectures,  ELMo

the  need  for  many  heavily-engineered  task advances  the  state  of  the  art  for  several  maj or  NLP

specific  architectures .  BERT  is  the  first  fine benchmarks  (Peters  et  al. ,  20 1 8a)  including  ques

tuning  based  representation  model  that  achieves
 tion  answering  (Rajpurkar  et  al. ,  20 1 6) ,  sentiment

state-of-the-art  performance  on  a  large  suite
 analysis  (S ocher  et  al. ,  20 1 3 ) ,  and  named  entity

of  sentence-level  and  token-level  tasks,  outper recognition  (Tj ong  Kim  S ang  and  De  Meulder,

forming  many  task-specific  architectures .
 2003) .  Melamud  et  al.  (20 1 6)  proposed  learning

contextual  representations  through  a  task  to  pre

•  BERT  advances  the  state  of  the  art  for  eleven

dict  a  single  word  from  both  left  and  right  context

NLP  tasks .  The  code  and  pre-trained  mod

using  LSTMs .  Similar  to  ELMo,  their  model  is

els  are  available  at  ht t p s : / / g i t hub . c om / 
 -

feature based  and  not  deeply  bidirectional.  Fedus

go o g l e - r e s e a r ch / b e rt .

et  al .  (20 1 8)  shows  that  the  cloze  task  can  be  used

2  Related  Work
 to  improve  the  robustness  of  text  generation  mod

el s .

There  is  a  long  history  of  pre-training  general  lan

guage  representations,  and  we  briefly  review  the
 2.2  Unsupervised  Fine-tuning  Approaches

most  widely-used  approaches  in  this  section.

As  with  the  feature-based  approaches,  the  first

2.1  Unsupervised  Feature-based  Approaches
 works  in  this  direction  only  pre-trained  word  em

Learning  widely  applicable  representations  of
 bedding  parameters  from  unlabeled  text  (Col

words  has  been  an  active  area  of  research  for
 lobert  and  Weston,  2008) .

decades,  including  non-neural  (Brown  et  al. ,  1 992 ;
 More  recently,  sentence  or  document  encoders

Ando  and  Zhang,  2005 ;  Blitzer  et  al. ,  2006)  and
 which  produce  contextual  token  representations

neural  (Mikolov  et  al. ,  20 1 3 ;  Pennington  et  al. ,
 have  been  pre-trained  from  unlabeled  text  and

20 1 4)  methods .  Pre-trained  word  embeddings
 fine-tuned  for  a  supervised  downstream  task  (Dai

are  an  integral  part  of  modern  NLP  systems,  of and  Le,  20 1 5 ;  Howard  and  Ruder,  20 1 8 ;  Radford

fering  significant  improvements  over  embeddings
 et  al. ,  20 1 8) .  The  advantage  of  these  approaches

learned  from  scratch  (Turian  et  al. ,  20 1 0) .  To  pre is  that  few  parameters  need  to  be  learned  from

train  word  embedding  vectors ,  left-to-right  lan scratch.  At  least  partly  due  to  this  advantage,

guage  modeling  obj ectives  have  been  used  (Mnih
 OpenAI  GPT  (Radford  et  al. ,  20 1 8)  achieved  pre

and  Hinton,  2009) ,  as  well  as  obj ectives  to  dis viously  state-of-the-art  results  on  many  sentence

criminate  correct  from  incorrect  words  in  left  and
 level  tasks  from  the  GLUE  benchmark  (Wang

right  context  (Mikolov  et  al. ,  20 1 3 ) .
 et  al. ,  20 1 8 a) .  Left-to-right  language  model-

#### N S P   Mask LM   Mask LM 
 M N LI   N E R S
 QuAD 
 Start/End Span

C   T 1   . . .   T N 
 T [S E P]   T 1 ’   
 . . .   T M ’ 
 
 C   T 1   . . .   T N 
 T [S E P]   T 1 ’   
 . . .   T M ’

## B E RT 
 B E RT  B E RT

E [C LS]   E 1 
 . . .   E N 
   E [S E P]   E 1 ’   
 . . .   E M ’ 
 
 E [C LS]   E 1 
 . . .   E N 
   E [S E P]   E 1 ’   
 . . .   E M ’

[C LS]   To k  1 . . .   To k N    [S E P]   To k  1   . . .   To kM 
 [C LS]   To k  1 . . .   To k N    [S E P]   To k  1   . . .   To kM

### Masked Sentence A   Masked Sentence B 
 Question   Parag raph

### U n labeled Sentence A and B Pai r  
 Question Answer Pai r

# P re-tra i n i n g   F i n e-Tu n i n g

Figure   1 :  Overall  pre-training  and  fine-tuning  procedures  for  BERT.  Apart  from  output  layers,  the  same  architec

tures  are  used  in  both  pre-training  and  fine-tuning .  The  same  pre-trained  model  parameters  are  used  to  initialize

models  for  different  down- stream  tasks .  During  fine-tuning,  all  parameters  are  fine-tuned.  [ C L S ]   is  a  special

symbol  added  in  front  of  every  input  example,  and  [ S E P ]   is  a  special  separator  token  (e. g .  separating  ques

tions/answers) .

ing  and  auto-encoder  obj ectives  have  been  used
 mal  difference  between  the  pre-trained  architec

for  pre-training  such  models  (Howard  and  Ruder,
 ture  and  the  final  downstream  architecture.

20 1 8 ;  Radford  et  al . ,  20 1 8 ;  Dai  and  Le,  20 1 5 ) .

Model  Architecture  BERT’s  model  architec

2.3  Transfer  Learning  from  Supervised  Data
 ture  is  a  multi-layer  bidirectional  Transformer  en

 coder  based  on  the  original  implementation  de

There  has  also  been  work  showing  effective  trans

scribed  in  Vaswani  et  al .  (20 1 7)  and  released  in

fer  from  supervised  tasks  with  large  datasets ,  such

the  t e n s o r 2 t e n s o r  library. 1  Because  the  use

as  natural  language  inference  (Conneau  et  al. ,

of  Transformers  has  become  common  and  our  im

20 1 7)  and  machine  translation  (McCann  et  al. ,

 plementation  is  almost  identical  to  the  original,

20 1 7) .  Computer  vision  research  has  also  demon

we  will  omit  an  exhaustive  background  descrip

strated  the  importance  of  transfer  learning  from

- tion  of  the  model  architecture  and  refer  readers  to

large  pre trained  models,  where  an  effective  recipe

- -  Vaswani  et  al .  (20 1 7)  as  well  as  excellent  guides

is  to  fine tune  models  pre trained  with  Ima “ ”2

such  as   The  Annotated  Transformer.

geNet  (Deng  et  al . ,  2009 ;  Yosinski  et  al . ,  20 1 4) .

In  this  work,  we  denote  the  number  of  layers

3  BERT
 (i . e. ,  Transformer  blocks)  as  L ,  the  hidden  size  as

H,  and  the  number  of  self-attention  heads  as  A .
3

We  introduce  BERT  and  its  detailed  implementa We  primarily  report  results  on  two  model  sizes :

tion  in  this  section.  There  are  two  steps  in  our
 BERTBASE  (L= 1 2,  H=768 ,  A= 1 2,  Total  Param

framework:  pre-training  and  fine-tuning.  Dur eters= 1 1 0M)  and  BERTLARGE  (L=24,  H= 1 024,

ing  pre-training,  the  model  is  trained  on  unlabeled
 A= 1 6,  Total  Parameters=340M) .

data  over  different  pre-training  tasks .  For  fine BERTBASE  was  chosen  to  have  the  same  model

tuning,  the  BERT  model  is  first  initialized  with
 size  as  OpenAI  GPT  for  comparison  purposes .

the  pre-trained  parameters,  and  all  of  the  param Critically,  however,  the  BERT  Transformer  uses

eters  are  fine-tuned  using  labeled  data  from  the
 bidirectional  self-attention,  while  the  GPT  Trans

downstream  tasks .  Each  downstream  task  has  sep former  uses  constrained  self-attention  where  every

arate  fine-tuned  models ,  even  though  they  are  ini token  can  only  attend  to  context  to  its  left.4

tialized  with  the  same  pre-trained  parameters .  The

question-answering  example  in  Figure  1   will  serve
 1
https ://github.com/tensorflow/tensor2tensor

as  a  runnin  exam le  for  this  section .
 2
http ://nlp. seas .harvard.edu/20 1 8/04/03/attention.html

g p 3
In  all  cases  we  set  the  feed-forward/filter  size  to  be  4H,

A  distinctive  feature  of  B ERT  is  its  unified  ar i.e. ,  3072  for  the  H  =  768  and  4096  for  the  H  =  1 024 .

chitecture  acros s  different  tasks .  There  is  mini 4We  note  that  in  the  literature  the  bidirectional  Trans-

Input/Output  Representations  To  make  BERT
 In  order  to  train  a  deep  bidirectional  representa

handle  a  variety  of  down- stream  tasks,  our  input
 tion,  we  simply  mask  some  percentage  of  the  input

representation  is  able  to  unambiguously  represent
 tokens  at  random,  and  then  predict  those  masked

“

both  a  single  sentence  and  a  pair  of  sentences
 tokens .  We  refer  to  this  procedure  as  a   masked

”

(e. g . ,  h  Question,  Answer i )  in  one  token  sequence.
 LM  (MLM) ,  although  it  is  often  referred  to  as  a

Throughout  this  work,  a  “sentence”  can  be  an  arbi Cloze  task  in  the  literature  (Taylor,  1 95 3 ) .  In  this

trary  span  of  contiguous  text,  rather  than  an  actual
 case,  the  final  hidden  vectors  corresponding  to  the

linguistic  sentence.  A  “sequence”  refers  to  the  in mask  tokens  are  fed  into  an  output  softmax  over

put  token  sequence  to  BERT,  which  may  be  a  sin the  vocabulary,  as  in  a  standard  LM .  In  all  of  our

gle  sentence  or  two  sentences  packed  together.
 experiments,  we  mask   1 5 %  of  all  WordPiece  to

We  use  WordPiece  embeddings  (Wu  et  al. ,
 kens  in  each  sequence  at  random.  In  contrast  to

20 1 6)  with  a  30,000  token  vocabulary.  The  first
 denoising  auto-encoders  (Vincent  et  al. ,  2008) ,  we

token  of  every  sequence  is  always  a  special  clas only  predict  the  masked  words  rather  than  recon

sification  token  ( [ C L S ] ) .  The  final  hidden  state
 structing  the  entire  input.

corresponding  to  this  token  is  used  as  the  ag Although  this  allows  us  to  obtain  a  bidirec

gregate  sequence  representation  for  classification
 tional  pre-trained  model,  a  downside  is  that  we

tasks .  Sentence  pairs  are  packed  together  into  a
 are  creating  a  mismatch  between  pre-training  and

single  sequence.  We  differentiate  the  sentences  in
 fine-tuning,  since  the  [ MA S K ]   token  does  not  ap

two  ways .  First,  we  separate  them  with  a  special
 pear  during  fine-tuning .  To  mitigate  this ,  we  do

token  ( [ S E P ] ) .  Second,  we  add  a  learned  embed not  always  replace  “masked”  words  with  the  ac

ding  to  every  token  indicating  whether  it  belongs
 tual  [ MA S K ]   token.  The  training  data  generator

to  sentence  A  or  sentence  B .  As  shown  in  Figure  1 ,
 chooses   1 5 %  of  the  token  positions  at  random  for

we  denote  input  embedding  as  E,  the  final  hidden
 prediction.  If  the  i-th  token  is  chosen,  we  replace

vector  of  the  special  [ C L S ]   token  as  C  ∈  R
H ,
 the  i -th  token  with  ( 1 )  the  [ MA S K ]   token  80%  of

and  the  final  hidden  vector  for  the  i
th  input  token
 the  time  (2)  a  random  token   1 0%  of  the  time  (3 )

as  Ti  ∈  R
H .
 the  unchanged  i -th  token   1 0%  of  the  time.  Then,

For  a  given  token,  its  input  representation  is
 Ti  will  be  used  to  predict  the  original  token  with

constructed  by  summing  the  corresponding  token,
 cross  entropy  loss .  We  compare  variations  of  this

segment,  and  position  embeddings .  A  visualiza procedure  in  Appendix  C .2.

tion  of  this  construction  can  be  seen  in  Figure  2 .

Task  #2:  Next  Sentence  Prediction  (NSP)

3.1  Pre-training  BERT
 Many  important  downstream  tasks  such  as  Ques

Unlike  Peters  et  al.  (20 1 8a)  and  Radford  et  al.
 tion  Answering  (QA)  and  Natural  Language  Infer

(20 1 8) ,  we  do  not  use  traditional  left-to-right  or
 ence  (NLI)  are  based  on  understanding  the  rela

right-to-left  language  models  to  pre-train  BERT.
 tionship  between  two  sentences,  which  is  not  di

Instead,  we  pre-train  BERT  using  two  unsuper rectly  captured  by  language  modeling.  In  order

vised  tasks ,  described  in  this  section.  This  step
 to  train  a  model  that  understands  sentence  rela

is   resented  in  the  left   art  of  Fi ure  1 .
 tionships ,  we  pre-train  for  a  binarized  next  sen

p p g

tence p  rediction  task  that  can  be  trivially  gener

Task  #1 :  Masked  LM  Intuitively,  it  is  reason ated  from  any  monolingual  corpus .  Specifically,

able  to  believe  that  a  deep  bidirectional  model  is
 when  choosing  the  sentences  A  and  B  for  each  pre

strictly  more  powerful  than  either  a  left-to-right
 training  example,  50%  of  the  time  B  is  the  actual

model  or  the  shallow  concatenation  of  a  left-to next  sentence  that  follows  A  (labeled  as  I s Ne xt ) ,

right  and  a  right-to-left  model.  Unfortunately,
 and  50%  of  the  time  it  is  a  random  sentence  from

standard  conditional  language  models  can  only  be
 the  corpus  (labeled  as  Not Ne xt ) .  As  we  show

trained  left-to-right  or  right-to-left,  since  bidirec in  Figure  1 ,  C  is  used  for  next  sentence  predic

tional  conditioning  would  allow  each  word  to  in tion  (NSP) .5  Despite  its  simplicity,  we  demon

directly  “see  itself” ,  and  the  model  could  trivially
 strate  in  S ection  5 . 1   that  pre-training  towards  this

predict  the  target  word  in  a  multi-layered  context.
 task  is  very  beneficial  to  both  QA  and  NLI.  6

former  is  often  referred  to  as  a  “Transformer  encoder”  while
 5 The  final  model  achieves  97 %-98 %  accuracy  on  NSP.

the  left-context-only  version  is  referred  to  as  a  “Transformer
 6The  vector  C  is  not  a  meaningful  sentence  representation

decoder”  since  it  can  be  used  for  text  generation.
 without  fine-tuning,  since  it  was  trained  with  NSP.

## I n p u t 
 [CLS]   m y   d o g   i s   c u te   [ S E P ]   h e   l i ke s   p l a y   # # i n g   [ S E P ]

# TEomkbeend d 
i n g s 
 E [CLS]   E my   E d og   E i s   E cute   E [ S E P] 
 E h e   E l i kes   E p l ay   E # # i n g   E [ S E P]

# Seg ment

## E m bed d i n g s 
 EA 
 EA 
 EA 
 EA 
 EA 
 EA 
 E B 
 E B 
 E B 
 E B 
 E B

# Pos i ti o n

## E m bed d i n g s 
 E 0   E 1 
 E 2 
 E 3 
 E 4 
 E 5 
 E 6 
 E 7 
 E 8 
 E 9 
 E 1 0

Figure  2 :  BERT  input  representation.  The  input  embeddings  are  the  sum  of  the  token  embeddings,  the  segmenta

tion  embeddings  and  the  position  embeddings .

The  NSP  task  is  closely  related  to  representation (4)  a  degenerate  text- ∅  pair  in  text  classification

learning  obj ectives  used  in  Jernite  et  al.  (20 1 7)  and
 or  sequence  tagging .  At  the  output,  the  token  rep

Logeswaran  and  Lee  (20 1 8) .  However,  in  prior
 resentations  are  fed  into  an  output  layer  for  token

work,  only  sentence  embeddings  are  transferred  to
 level  tasks ,  such  as  sequence  tagging  or  question

down-stream  tasks,  where  BERT  transfers  all  pa answering,  and  the  [ C L S ]   representation  is  fed

rameters  to  initialize  end-task  model  parameters .
 into  an  output  layer  for  classification,  such  as  en

tailment  or  sentiment  analysis .

Pre-training  data  The  pre-training  procedure
 - - 

Compared  to  pre training,  fine tuning  is  rela

largely  follows  the  existing  literature  on  language
 

tively  inexpensive.  All  of  the  results  in  the  pa

model  pre-training .  For  the  pre-training  corpus  we
 

per  can  be  replicated  in  at  most   1  hour  on  a  sin

use  the  B ooksCorpus  (800M  words)  (Zhu  et  al. ,

gle  Cloud  TPU,  or  a  few  hours  on  a  GPU,  starting

20 1 5)  and  English  Wikipedia  (2,500M  words) .
 from  the  exact  same   re-trained  model.7  We  de

p

For  Wikipedia  we  extract  only  the  text  passages
 - 

scribe  the  task specific  details  in  the  correspond

and  ignore  lists ,  tables ,  and  headers .  It  is  criti

ing  subsections  of  Section  4 .  More  details  can  be

cal  to  use  a  document-level  corpus  rather  than  a

found  in  Appendix  A. 5 .

shuffled  sentence-level  corpus  such  as  the  Billion

Word  Benchmark  (Chelba  et  al. ,  20 1 3)  in  order  to
 4  Experiments

extract  long  contiguous  sequences .

In  this  section,  we  present  BERT  fine-tuning  re

3.2  Fine-tuning  BERT
 sults  on   1 1  NLP  tasks .

Fine-tuning  is  straightforward  since  the  self 4. 1  GLUE

attention  mechanism  in  the  Transformer  al

The  General  Language  Understanding  Evaluation

lows  BERT  to  model  many  downstream  tasks—
 

(GLUE)  benchmark  (Wang  et  al. ,  20 1 8a)  is  a  col

whether  they  involve  single  text  or  text  pairs—by

lection  of  diverse  natural  language  understanding

swapping  out  the  appropriate  inputs  and  outputs .

tasks .  Detailed  descriptions  of  GLUE  datasets  are

For  applications  involving  text  pairs,  a  common

included  in  Appendix  B . 1 .

pattern  is  to  independently  encode  text  pairs  be -

To  fine tune  on  GLUE,  we  represent  the  input

fore  applying  bidirectional  cross  attention,  such

sequence  (for  single  sentence  or  sentence  pairs)

as  Parikh  et  al .  (20 1 6) ;  S eo  et  al .  (20 1 7) .  B ERT
 

as  described  in  Section  3 ,  and  use  the  final  hid

instead  uses  the  self-attention  mechanism  to  unify
 H

den  vector  C  ∈  R
 corresponding  to  the  first

these  two  stages ,  as  encoding  a  concatenated  text
 

input  token  ( [ C L S ] )  as  the  aggregate  representa

pair  with  self-attention  effectively  includes  bidi

tion.  The  only  new  parameters  introduced  during

rectional  cross  attention  between  two  sentences .
 -

fine tuning  are  classification  layer  weights  W  ∈

For  each  task,  we  simply  plug  in  the  task K × H 

R
 ,  where  K  is  the  number  of  labels .  We  com

specific  inputs  and  outputs  into  BERT  and  fine

pute  a  standard  classification  loss  with  C  and  W,

tune  all  the  parameters  end-to-end.  At  the  in T

i.e. ,  log (softmax ( CW ) ) .

put,  sentence  A  and  sentence  B  from  pre-training

are  analo ous  to  ( 1 )  sentence   airs  in   ara hras 7
For  example,  the  BERT  SQuAD  model  can  be  trained  in

g p p p around  30  minutes  on  a  single  Cloud  TPU  to  achieve  a  Dev

ing ,  (2)  hypothesis-premise  pairs  in  entailment,  (3 )
 F 1  score  of  9 1 .0% .

question-passage  pairs  in  question  answering,  and
 8
See  ( 1 0)  in  ht t p s : / / g l u eb e n chma r k . c om / f a q.

System  MNLI-(m/mm)  QQP  QNLI  SST-2  CoLA  STS-B  MRPC  RTE  Average

3 92k  3 63k   1 08k  67k  8 . 5k  5 . 7k  3 . 5k  2 . 5k  -

Pre-OpenAI  S OTA  80 . 6/80 . 1  66 . 1  82 . 3  93 . 2  3 5 . 0  8 1 . 0  86 . 0  6 1 .7  74 . 0

BiLSTM+ELMo+Attn  76 .4/76 . 1  64. 8  79 . 8  90.4  3 6 .0  73 . 3  84. 9  56 . 8  7 1 .0

OpenAI  GPT  82 . 1 /8 1 .4  70 . 3  87 .4  9 1 . 3  45 .4  80 . 0  82 . 3  5 6 . 0  75 . 1

BERTBASE  84 . 6/8 3 .4  7 1 . 2  90 . 5  93 . 5  5 2 . 1  85 . 8  8 8 . 9  66 .4  79 . 6

BERTLARGE  86.7/85.9  72. 1  92.7  94.9  60.5  86.5  89.3  70. 1  82. 1

Table   1 :  GLUE  Test  results,  scored  by  the  evaluation  server  (ht t p s : / / g l u eb e n chma r k . c om / l e a de rb o a r d) .

The  number  below  each  task  denotes  the  number  of  training  examples .  The  “Average”  column  is  slightly  different

than  the  official  GLUE  score,  since  we  exclude  the  problematic  WNLI  set. 8  BERT  and  OpenAI  GPT  are  single

model,  single  task.  F 1  scores  are  reported  for  QQP  and  MRPC,  Spearman  correlations  are  reported  for  STS -B ,  and

accuracy  scores  are  reported  for  the  other  tasks .  We  exclude  entries  that  use  BERT  as  one  of  their  components .

We  use  a  batch  size  of  3 2  and  fine-tune  for  3
 Wikipedia  containing  the  answer,  the  task  is  to

epochs  over  the  data  for  all  GLUE  tasks .  For  each
 predict  the  answer  text  span  in  the  passage.

task,  we  selected  the  best  fine-tuning  learning  rate
 As  shown  in  Figure  1 ,  in  the  question  answer

(among  5e-5 ,  4e-5 ,  3 e-5 ,  and  2e-5)  on  the  Dev  set.
 ing  task,  we  represent  the  input  question  and  pas

Additionally,  for  BERTLARGE  we  found  that  fine sage  as  a  single  packed  sequence,  with  the  ques

tuning  was  sometimes  unstable  on  small  datasets,
 tion  using  the  A  embedding  and  the  passage  using

so  we  ran  several  random  restarts  and  selected  the
 the  B  embedding .  We  only  introduce  a  start  vec

best  model  on  the  Dev  set.  With  random  restarts ,
 tor  S  ∈  R
H  and  an  end  vector  E  ∈  R
H  during

we  use  the  same  pre-trained  checkpoint  but  per fine-tuning .  The  probability  of  word  i  being  the

form  different  fine-tuning  data  shuffling  and  clas start  of  the  answer  span  is  computed  as  a  dot  prod

sifier  layer  initialization.9
 uct  between  Ti  and  S  followed  by  a  softmax  over

Results  are  presented  in  Table  1 .  B oth
 all  of  the  words  in  the  paragraph :   Pi  =
 e
 
S · STi·  T .

 P j
 e
 j

BERTBASE  and  BERTLARGE  outperform  all  sys

The  analogous  formula  is  used  for  the  end  of  the

tems  on  all  tasks  by  a  substantial  margin,  obtaining

answer  span.  The  score  of  a  candidate  span  from

4.5 %  and  7 .0%  respective  average  accuracy  im · ·

position  i  to  position  j  is  defined  as  S Ti  +  E Tj  ,

provement  over  the  prior  state  of  the  art.  Note  that

and  the  maximum  scoring  span  where  j  ≥  i  is

BERTBASE  and  OpenAI  GPT  are  nearly  identical

used  as  a  prediction.  The  training  obj ective  is  the

in  terms  of  model  architecture  apart  from  the  at -

sum  of  the  log likelihoods  of  the  correct  start  and

tention  masking .  For  the  largest  and  most  widely
 -

end  positions .  We  fine tune  for  3  epochs  with  a

reported  GLUE  task,  MNLI,  BERT  obtains  a  4.6%
 -

learning  rate  of  5 e 5  and  a  batch  size  of  3 2 .

absolute  accuracy  improvement.  On  the  official

1 0 Table  2  shows  top  leaderboard  entries  as  well

GLUE  leaderboard ,  BERTLARGE  obtains  a  score

as  results  from  top  published  systems  (S eo  et  al . ,

of  80.5 ,  compared  to  OpenAI  GPT,  which  obtains

20 1 7 ;  Clark  and  Gardner,  20 1 8 ;  Peters  et  al . ,

72 . 8  as  of  the  date  of  writing .

20 1 8 a;  Hu  et  al . ,  20 1 8) .  The  top  results  from  the

We  find  that  BERTLARGE  significantly  outper - -

SQuAD  leaderboard  do  not  have  up to date  public

forms  BERTBASE  across  all  tasks ,  especially  those
 1 1

system  descriptions  available, and  are  allowed  to

with  very  little  training  data.  The  effect  of  model

use  any  public  data  when  training  their  systems .

size  is  explored  more  thoroughly  in  Section  5 . 2 .

We  therefore  use  modest  data  augmentation  in

4.2  S uAD  v1. 1
 our  system  by  first  fine-tuning  on  TriviaQA  (Joshi

Q

et  al. ,  20 1 7)  befor  fine-tuning  on  S QuAD .

The  Stanford  Question  Answering  Dataset

Our  best  performing  system  outperforms  the  top

(S QuAD  v 1 . 1 )  is  a  collection  of   1 00k  crowd

leaderboard  system  by  + 1 . 5  F 1  in  ensembling  and

sourced  question/answer  pairs  (Rajpurkar  et  al. ,

+ 1 . 3  F 1  as  a  single  system.  In  fact,  our  single

20 1 6) .  Given  a  question  and  a  passage  from
 

BERT  model  outperforms  the  top  ensemble  sys

9The  GLUE  data  set  distribution  does  not  include  the  Test
 tem  in  terms  of  F 1  score.  Without  TriviaQA  fine

labels,  and  we  only  made  a  single  GLUE  evaluation  server

submission  for  each  of  BERTBASE  and  BERTLARGE .
 1 1 QANet  is  described  in  Yu  et  al.  (20 1 8) ,  but  the  system

10https ://gluebenchmark.com/leaderboard
 has  improved  substantially  after  publication.

System  Dev  Test
 System  Dev  Test

EM  F 1  EM  F 1

ESIM+GloVe  5 1 .9  52.7

Top  Leaderboard  Systems  (Dec   1 0th,  20 1 8)
 ESIM+ELMo  59 . 1  59 .2

Human  -  -  82 . 3  9 1 . 2
 OpenAI  GPT  -  7 8 . 0

# 1  Ensemble  -  nlnet  -  -  8 6 . 0  9 1 . 7
 -

#2  Ensemble  -  QANet  -  -  84 . 5  90 . 5
 BERTBASE  8 1 . 6

BERTLARGE  86.6  86.3

Published
 †
 -

BiDAF+ELMo  (Single)  -  85 . 6  -  85 . 8
 Human  (expert)  85 .0

R.M .  Reader  (Ensemble)  8 1 . 2  87 . 9  82 . 3  8 8 . 5
 Human  (5  annotations) †
 -  8 8 . 0

Ours
 † 

BERTBASE  (Single)  80. 8  8 8 .5  -  -
 Table  4 :  SWAG  Dev  and  Test  accuracies .  Human  per

BERTLARGE  (Single)  84. 1  90.9  -  -
 formance  is  measured  with   1 00  samples ,  as  reported  in

BERTLARGE  (Ensemble)  85 . 8  9 1 . 8  -  -
 the  SWAG  paper.

BERTLARGE  (Sgl.+TriviaQA)  84.2  91. 1  85. 1  91.8

BERTLARGE  (Ens .+TriviaQA)  86.2  92.2  87.4  93.2

siˆ,j  =  maxj ≥i S · Ti  +  E · Tj  .  We  predict  a  non-null

Table  2 :  SQuAD   1 . 1  results .  The  BERT  ensemble
 ˆ 

-  answer  when  si ,j  >  snull  +  τ  ,  where  the  thresh

is  7x  systems  which  use  different  pre training  check

- old  τ  is  selected  on  the  dev  set  to  maximize  F 1 .

points  and  fine tuning  seeds .

We  did  not  use  TriviaQA  data  for  this  model.  We

fine-tuned  for  2  epochs  with  a  learning  rate  of  5e-5

System  Dev  Test

EM  F 1  EM  F 1 
 and  a  batch  size  of  4 8 .

Top  Leaderboard  Systems  (Dec  1 0th,  20 1 8)
 The  results  compared  to  prior  leaderboard  en

Human  86 . 3  89 .0  86 .9  89 .5
 tries  and  top  published  work  (Sun  et  al . ,  20 1 8 ;

# 1  Single  -  MIR-MRC  (F-Net)  -  -  74. 8  7 8 .0
 Wang  et  al . ,  20 1 8b)  are  shown  in  Table  3 ,  exclud

#2  S ingle  -  nlnet  -  -  74 . 2  77 . 1

ing  systems  that  use  BERT  as  one  of  their  com

Published

unet  (Ensemble)  -  -  7 1 .4  74.9
 ponents .  We  observe  a  +5 . 1  F 1  improvement  over

SLQA+  (Single)  -  7 1 .4  74.4
 the  previous  best  system.

Ours

BERTLARGE  (Single)  7 8 .7  8 1 .9  80.0  83 . 1 
 4.4  SWAG

The  Situations  With  Adversarial  Generations

Table  3 :  SQuAD  2.0  results .  We  exclude  entries  that
 (SWAG)  dataset  contains   1 1 3k  sentence-pair  com

use  BERT  as  one  of  their  components.
 pletion  examples  that  evaluate  grounded  common

sense  inference  (Zellers  et  al . ,  20 1 8) .  Given  a  sen

-  tence,  the  task  is  to  choose  the  most  plausible  con

tuning  data,  we  only  lose  0 . 1 0 .4  F 1 ,  still  outper

1 2
 tinuation  among  four  choices .

forming  all  existing  systems  by  a  wide  margin.

When  fine-tuning  on  the  SWAG  dataset,  we

4.3  SQuAD  v2.0
 construct  four  input  sequences,  each  containing

the  concatenation  of  the  given  sentence  (sentence

The  SQuAD  2.0  task  extends  the  SQuAD   1 . 1

A)  and  a  possible  continuation  (sentence  B) .  The

problem  definition  by  allowing  for  the  possibility
 - 

only  task specific  parameters  introduced  is  a  vec

that  no  short  answer  exists  in  the  provided  para

tor  whose  dot  product  with  the  [ C L S ]   token  rep

graph,  making  the  problem  more  realistic .

resentation  C  denotes  a  score  for  each  choice

We  use  a  simple  approach  to  extend  the  SQuAD

which  is  normalized  with  a  softmax  layer.

v 1 . 1  BERT  model  for  this  task.  We  treat  ques

We  fine-tune  the  model  for  3  epochs  with  a

tions  that  do  not  have  an  answer  as  having  an  an

learning  rate  of  2e-5  and  a  batch  size  of   1 6 .  Re

swer  span  with  start  and  end  at  the  [ C L S ]   to

sults  are  presented  in  Table  4.  BERTLARGE  out

ken.  The  probability  space  for  the  start  and  end
 ’

performs  the  authors  baseline  ESIM+ELMo  sys

answer  span  positions  is  extended  to  include  the

tem  by  +27 . 1 %  and  OpenAI  GPT  by  8 . 3 % .

position  of  the  [ C L S ]   token.  For  prediction,  we

compare  the  score  of  the  no-answer  span :  snull  =

S · C  +  E · C  to  the  score  of  the  best  non-null  span

5  Ablation  Studies

12 In  this  section,  we  perform  ablation  experiments

The  TriviaQA  data  we  used  consists  of  paragraphs  from

TriviaQA-Wiki  formed  of  the  first  400  tokens  in  documents,
 over  a  number  of  facets  of  BERT  in  order  to  better

that  contain  at  least  one  of  the  provided  possible  answers .
 understand  their  relative  importance .  Additional

Dev  Set
 results  are  still  far  worse  than  those  of  the  pre

Tasks  MNLI-m  QNLI  MRPC  SST-2  SQuAD
 trained  bidirectional  models .  The  BiLSTM  hurts

(Acc)  (Acc)  (Acc)  (Acc)  (F 1 )

performance  on  the  GLUE  tasks .

BERTBASE  84 .4  8 8 .4  86 .7  92 .7  8 8 . 5

No  NSP  83 .9  84.9  86.5  92. 6  87 .9
 We  recognize  that  it  would  also  be  pos sible  to

LTR  &  No  NSP  82. 1  84. 3  77 .5  92. 1  77 . 8
 train  separate  LTR  and  RTL  models  and  represent

+  BiLSTM  82. 1  84. 1  75 .7  9 1 .6  84.9
 each  token  as  the  concatenation  of  the  two  mod

els ,  as  ELMo  does .  However:  (a)  this  is  twice  as

Table  5 :  Ablation  over  the  pre-training  tasks  using  the

“ ” expensive  as  a  single  bidirectional  model ;  (b)  this

BERTBASE  architecture.   No  NSP  is  trained  without

the  next  sentence  prediction  task.  “LTR  &  No  NSP”  is
 is  non-intuitive  for  tasks  like  QA,  since  the  RTL

trained  as  a  left-to-right  LM  without  the  next  sentence
 model  would  not  be  able  to  condition  the  answer

prediction,  like  OpenAI  GPT.  “+  BiLSTM”  adds  a  ran on  the  question ;  (c)  this  it  is  strictly  les s  powerful

domly  initialized  BiLSTM  on  top  of  the  “LTR  +  No
 than  a  deep  bidirectional  model,  since  it  can  use

NSP”  model  during  fine-tuning.
 both  left  and  right  context  at  every  layer.

ablation  studies  can  be  found  in  Appendix  C .

5.2  Effect  of  Model  Size

In  this  section,  we  explore  the  effect  of  model  size

5. 1  Effect  of  Pre-training  Tasks
 on  fine-tuning  task  accuracy.  We  trained  a  number

 of  BERT  models  with  a  differing  number  of  layers,

We  demonstrate  the  importance  of  the  deep  bidi

 hidden  units ,  and  attention  heads ,  while  otherwise

rectionality  of  BERT  by  evaluating  two  pre

 using  the  same  hyperparameters  and  training  pro

training  obj ectives  using  exactly  the  same  pre

-  cedure  as  described  previously.

training  data,  fine tuning  scheme,  and  hyperpa

rameters  as  BERT :
 Results  on  selected  GLUE  tasks  are  shown  in

BASE

Table  6 .  In  this  table,  we  report  the  average  Dev

No  NSP:  A  bidirectional  model  which  is  trained
 Set  accuracy  from  5  random  restarts  of  fine-tuning .

using  the  “masked  LM”  (MLM)  but  without  the
 We  can  see  that  larger  models  lead  to  a  strict  ac

“ ”

next  sentence  prediction  (NSP)  task.
 curacy  improvement  across  all  four  datasets,  even

LTR  &  No  NSP:  A  left-context-only  model  which
 for  MRPC  which  only  has  3 ,600  labeled  train

is  trained  using  a  standard  Left-to-Right  (LTR)
 ing  examples ,  and  is  substantially  different  from

LM,  rather  than  an  MLM .  The  left-only  constraint
 the  pre-training  tasks .  It  is  also  perhaps  surpris

was  also  applied  at  fine-tuning,  because  removing
 ing  that  we  are  able  to  achieve  such  significant

it  introduced  a  pre-train/fine-tune  mismatch  that
 improvements  on  top  of  models  which  are  al

degraded  downstream  performance.  Additionally,
 ready  quite  large  relative  to  the  existing  literature.

this  model  was  pre-trained  without  the  NSP  task.
 For  example,  the  largest  Transformer  explored  in

This  is  directly  comparable  to  OpenAI  GPT,  but
 Vaswani  et  al.  (20 1 7)  is  (L=6,  H= 1 024,  A= 1 6)

using  our  larger  training  dataset,  our  input  repre with   1 00M  parameters  for  the  encoder,  and  the

sentation,  and  our  fine-tuning  scheme.
 largest  Transformer  we  have  found  in  the  literature

We  first  examine  the  impact  brought  by  the  NSP
 is  (L=64,  H=5 1 2,  A=2)  with  235M  parameters

task.  In  Table  5 ,  we  show  that  removing  NSP
 (Al-Rfou  et  al. ,  20 1 8) .  By  contrast,  BERTBASE

hurts  performance  significantly  on  QNLI,  MNLI,
 contains  1 1 0M  parameters  and  BERTLARGE  con

and  SQuAD   1 . 1 .  Next,  we  evaluate  the  impact
 tains  340M  parameters .

of  training  bidirectional  representations  by  com It  has  long  been  known  that  increasing  the

“ ” “ ”

paring   No  NSP  to   LTR  &  No  NSP .  The  LTR
 model  size  will  lead  to  continual  improvements

model  performs  worse  than  the  MLM  model  on  all
 on  large-scale  tasks  such  as  machine  translation

tasks,  with  large  drops  on  MRPC  and  SQuAD .
 and  language  modeling,  which  is  demonstrated

For  S QuAD  it  is  intuitively  clear  that  a  LTR
 by  the  LM  perplexity  of  held-out  training  data

model  will  perform  poorly  at  token  predictions,
 shown  in  Table  6 .  However,  we  believe  that

since  the  token-level  hidden  states  have  no  right this  is  the  first  work  to  demonstrate  convinc

side  context.  In  order  to  make  a  good  faith  at ingly  that  scaling  to  extreme  model  sizes  also

tempt  at  strengthening  the  LTR  system,  we  added
 leads  to  large  improvements  on  very  small  scale

a  randomly  initialized  BiLSTM  on  top .  This  does
 tasks,  provided  that  the  model  has  been  suffi

significantly  improve  results  on  S QuAD,  but  the
 ciently  pre-trained.  Peters  et  al.  (20 1 8b)  presented

mixed  results  on  the  downstream  task  impact  of
 System  Dev  F 1  Test  F 1

increasing  the  pre-trained  bi-LM  size  from  two
 ELMo  (Peters  et  al. ,  20 1 8a)  95 .7  92.2

to  four  layers  and  Melamud  et  al .  (20 1 6)  men CVT  (Clark  et  al. ,  20 1 8)  -  92.6

 CSE  (Akbik  et  al . ,  20 1 8)  -  93. 1

tioned  in  passing  that  increasing  hidden  dimen

sion  size  from  200  to  600  helped,  but  increasing
 Fine-tuning  approach

BERTLARGE  96.6  92. 8

further  to   1 ,000  did  not  bring  further  improve BERTBASE  96.4  92.4

ments .  B oth  of  these  prior  works  used  a  feature -

Feature based  approach  (BERTBASE)

based  approach —   we  hypothesize  that  when  the
 Embeddings  9 1 .0  -

model  is  fine-tuned  directly  on  the  downstream
 Second-to-Last  Hidden  95 .6  -

 Last  Hidden  94 . 9  -

tasks  and  uses  only  a  very  small  number  of  ran Weighted  Sum  Last  Four  Hidden  95 .9  -

domly  initialized  additional  parameters ,  the  task Concat  Last  Four  Hidden  96. 1  -

specific  models  can  benefit  from  the  larger,  more
 Weighted  Sum  All  1 2  Layers  95 .5  -

expressive  pre-trained  representations  even  when

downstream  task  data  is  very  small.

5.3  Feature-based  Approach  with  BERT

Table  7 :  CoNLL-2003  Named  Entity  Recognition  re

sults .  Hyperparameters  were  selected  using  the  Dev

set.  The  reported  Dev  and  Test  scores  are  averaged  over

5  random  restarts  using  those  hyperparameters .

All  of  the  BERT  results  presented  so  far  have  used

the  fine-tuning  approach,  where  a  simple  classifi

cation  layer  is  added  to  the  pre-trained  model,  and
 layer  in  the  output.  We  use  the  representation  of

all  parameters  are j  ointly  fine-tuned  on  a  down the  first  sub-token  as  the  input  to  the  token-level

stream  task.  However,  the  feature-based  approach,
 classifier  over  the  NER  label  set.

where  fixed  features  are  extracted  from  the  pre -

To  ablate  the  fine tuning  approach,  we  apply  the

trained  model,  has  certain  advantages .  First,  not
 - 

feature based  approach  by  extracting  the  activa

all  tasks  can  be  easily  represented  by  a  Trans -

tions  from  one  or  more  layers  without  fine tuning

former  encoder  architecture,  and  therefore  require
 

any  parameters  of  BERT.  These  contextual  em

a  task- specific  model  architecture  to  be  added.
 

beddings  are  used  as  input  to  a  randomly  initial

Second,  there  are  maj or  computational  benefits
 - -

ized  two layer  768 dimensional  BiLSTM  before

to  pre-compute  an  expensive  representation  of  the

the  clas sification  layer.

training  data  once  and  then  run  many  experiments

Results  are  presented  in  Table  7 .  BERTLARGE

with  cheaper  models  on  top  of  this  representation.

performs  competitively  with  state-of-the-art  meth

In  this  section,  we  compare  the  two  approaches

- ods .  The  best  performing  method  concatenates  the

by  applying  BERT  to  the  CoNLL 2003  Named

token  representations  from  the  top  four  hidden  lay

Entity  Recognition  (NER)  task  (Tj ong  Kim  Sang

ers  of  the  pre-trained  Transformer,  which  is  only

and  De  Meulder,  2003) .  In  the  input  to  BERT,  we

- 0. 3  F 1  behind  fine-tuning  the  entire  model.  This

use  a  case preserving  WordPiece  model,  and  we

demonstrates  that  BERT  is  effective  for  both  fine

include  the  maximal  document  context  provided

 tuning  and  feature-based  approaches .

by  the  data.  Following  standard  practice,  we  for

mulate  this  as  a  tagging  task  but  do  not  use  a  CRF

6  Conclusion

Hyperparams  Dev  Set  Accuracy

#L  #H  #A  LM  (ppl)  MNLI-m  MRPC  SST-2
 Recent  empirical  improvements  due  to  transfer

3  768   1 2  5 . 84  77 .9  79 . 8  8 8 .4
 learning  with  language  models  have  demonstrated

6  768  3  5 .24  80. 6  82.2  90.7
 that  rich,  unsupervised  pre-training  is  an  integral

6  768   1 2  4.68  8 1 .9  84. 8  9 1 . 3
 part  of  many  language  understanding  systems .  In

1 2  7 6 8   1 2  3 . 99  84 . 4  8 6 . 7  92 . 9

1 2   1 024   1 6  3 .54  85 .7  86.9  93 . 3
 particular,  these  results  enable  even  low-resource

24   1 024   1 6  3 .23  86.6  87 . 8  93 .7
 tasks  to  benefit  from  deep  unidirectional  architec

tures .  Our  maj or  contribution  is  further  general

Table  6 :  Ablation  over  BERT  model  size.  #L  =  the
 izing  these  findings  to  deep  bidirectional  architec

number  of  layers ;  #H  =  hidden  size;  #A  =  number  of  at tures ,  allowing  the  same  pre-trained  model  to  suc

tention  heads .  “LM  (ppl)”  is  the  masked  LM  perplexity
 cessfully  tackle  a  broad  set  of  NLP  tasks .

of  held-out  training  data.

References
 Kevin  Clark,  Minh-Thang  Luong,  Christopher  D  Man

ning,  and  Quoc  Le.  20 1 8 .  Semi-supervised  se

Alan  Akbik,  Duncan  Blythe,  and  Roland  Vollgraf.
 quence  modeling  with  cross-view  training.  In  Pro

20 1 8 .  Contextual  string  embeddings  for  sequence
 ceedings  of the  201 8  Conference  on  Empirical M  eth

labeling.  In  Proceedings  of  the  2 7th I  nternational
 ods  in N  atural L  anguage  Processing,  pages   1 9 1 4–

Conference  on  Computational L  inguistics,  pages
 1 925 .

1 63 8– 1 649 .

Ronan  Collobert  and  Jason  Weston.  2008 .  A  unified

Rami  Al-Rfou,  Dokook  Choe,  Noah  Constant,  Mandy
 architecture  for  natural  language  processing :  Deep

Guo,  and  Llion  Jones .  20 1 8 .  Character-level  lan neural  networks  with  multitask  learning.  In  Pro

guage  modeling  with  deeper  self-attention.  arXiv
 ceedings  of  the  25th  international  conference  on

preprint  arXiv: 1 808. 04444.
 Machine  learning,  pages   1 60– 1 67 .  ACM.

¨

Rie  Kubota  Ando  and  Tong  Zhang.  2005 .  A  framework
 Alexis  Conneau,  Douwe  Kiela,  Holger  Schwenk,  Loıc

for  learning  predictive  structures  from  multiple  tasks
 B arrault,  and  Antoine  B ordes .  20 1 7 .  Supervised

and  unlabeled  data.  Journal  of M  achine L  earning
 learning  of  universal  sentence  representations  from

Research,  6(Nov) : 1 8 1 7– 1 85 3 .
 natural  language  inference  data.  In  Proceedings  of

the  201 7  Conference  on  Empirical M  ethods  in N  at

ural L  anguage  Processing,  pages  670–680,  Copen

Luisa  Bentivogli,  Bernardo  Magnini,  Ido  Dagan,

hagen,  Denmark.  Association  for  Computational

Hoa  Trang  Dang,  and  Danilo  Giampiccolo.  2009 .

Linguistics .

The  fifth  PASCAL  recognizing  textual  entailment

challenge.  In  TAC.  NIST.
 -

Andrew  M  Dai  and  Quoc  V  Le.  20 1 5 .  Semi supervised

sequence  learning.  In  Advances  in  neural  informa

John  Blitzer,  Ryan  McDonald,  and  Fernando  Pereira.
 tion p  rocessing  systems,  pages  3079–3087 .

2006.  Domain  adaptation  with  structural  correspon

dence  learning .  In  Proceedings  of  the  2006  confer J.  Deng,  W.  Dong,  R.  Socher,  L. -J.  Li,  K.  Li,  and  L.  Fei

ence  on  empirical  methods  in  natural  language p  ro Fei.  2009 .  ImageNet:  A  Large-Scale  Hierarchical

cessing,  pages   1 20– 1 28 .  Association  for  Computa Ima e  Database.  In  CVPR09.

g

tional  Linguistics .

William  B  Dolan  and  Chris  Brockett.  2005 .  Automati

S amuel  R.  B owman,  Gabor  Angeli,  Christopher  Potts,
 cally  constructing  a  corpus  of  sentential  paraphrases .

and  Christopher  D .  Manning.  20 1 5 .  A  large  anno In  Proceedings  of  the  Third I  nternational  Workshop

tated  corpus  for  learning  natural  language  inference.
 on  Paraphrasing  (IWP2005) .

In  EMNLP.  Association  for  Computational  Linguis

tics .
 William  Fedus,  Ian  Goodfellow,  and  Andrew  M  Dai.

20 1 8 .  Maskgan :  B etter  text  generation  via  filling  in

Peter  F  Brown,  Peter  V  Desouza,  Robert  L  Mercer,
 the  .  arXiv p  reprint  arXiv: 1 801 . 07736.

Vincent  J  Della  Pietra,  and  Jenifer  C  Lai .   1 992 .

Class-based  n-gram  models  of  natural  language.
 Dan  Hendrycks  and  Kevin  Gimpel.  20 1 6.  Bridging

Computational  linguistics,   1 8 (4) :467–479 .
 nonlinearities  and  stochastic  regularizers  with  gaus

sian  error  linear  units .  CoRR,  abs/ 1 606 . 084 1 5 .

Daniel  Cer,  Mona  Diab,  Eneko  Agirre,  Inigo  Lopez

Gaz io,  and  Lucia  S ecia.  20 1 7 .  Semeval-20 1 7
 Felix  Hill,  Kyunghyun  Cho,  and  Anna  Korhonen.  20 1 6 .

p p

task   1 :  Semantic  textual  similarit  multilin ual  and
 Learning  distributed  representations  of  sentences

y g

crosslin ual  focused  evaluation.  In  Proceedings
 from  unlabelled  data.  In  Proceedings  of  the  201 6

g

of  the  1 1 th I  nternational  Workshop  on  Semantic
 Conference  of  the N  orth A  merican  Chapter  of  the

Evaluation  (SemEval-201 7),   a es   1– 1 4,  Vancou Association f  or  Computational L  inguistics: H  uman

ver,  Canada.  Association  forp  Cgom utational  Lin Language  Technologies.  Association  for  Computa

p

uistic s .
 tional  Linguistic s .

g

Jeremy  Howard  and  Sebastian  Ruder.  20 1 8 .  Universal

Ciprian  Chelba,  Tomas  Mikolov,  Mike  Schuster,  Qi  Ge,
 -

language  model  fine tuning  for  text  classification.  In

Thorsten  Brants,  Phillipp  Koehn,  and  Tony  Robin

ACL.  Association  for  Computational  Linguistics .

son.  20 1 3 .  One  billion  word  benchmark  for  measur

ing  progress  in  statistical  language  modeling .  arXiv

Minghao  Hu,  Yuxing  Peng,  Zhen  Huang,  Xipeng  Qiu,

preprint  arXiv: 1 31 2. 3005.

Furu  Wei,  and  Ming  Zhou.  20 1 8 .  Reinforced

mnemonic  reader  for  machine  reading  comprehen

Z.  Chen,  H.  Zhang,  X.  Zhang,  and  L.  Zhao .  20 1 8 .
 sion.  In  IJCAI.

Quora  question  pairs .

Yacine  Jernite,  Samuel  R.  Bowman,  and  David  Son

Christopher  Clark  and  Matt  Gardner.  20 1 8 .  Simple
 tag .  20 1 7 .  Discourse-based  obj ectives  for  fast  un

and  effective  multi-paragraph  reading  comprehen supervised  sentence  representation  learning.  CoRR,

sion.  In  ACL.
 abs/ 1 705 .00557 .

Mandar  Joshi,  Eunsol  Choi,  Daniel  S  Weld,  and  Luke
 Matthew  Peters,  Mark  Neumann,  Luke  Zettlemoyer,

Zettlemoyer.  20 1 7 .  Triviaqa:  A  large  scale  distantly
 and  Wen-tau  Yih.  20 1 8b .  Dissecting  contextual

supervised  challenge  dataset  for  reading  comprehen word  embeddings :  Architecture  and  representation.

sion.  In  ACL.
 In  Proceedings  of  the  201 8  Conference  on  Empiri

cal M  ethods  in N  atural L  anguage  Processing,  pages

Ryan  Kiros,  Yukun  Zhu,  Ruslan  R  Salakhutdinov,
 1 499– 1 509 .

Richard  Zemel,  Raquel  Urtasun,  Antonio  Torralba,

and  S anj a  Fidler.  20 1 5 .  Skip-thought  vectors .  In
 Alec  Radford,  Karthik  Narasimhan,  Tim  S alimans,  and

Advances  in  neural  information p  rocessing  systems,
 Ilya  Sutskever.  20 1 8 .  Improving  language  under

pages  3294–3 302.
 standing  with  unsupervised  learning.  Technical  re

port,  OpenAI.

Quoc  Le  and  Tomas  Mikolov.  20 1 4.  Distributed  rep

resentations  of  sentences  and  documents .  In  Inter Pranav  Rajpurkar,  Jian  Zhang,  Konstantin  Lopyrev,  and

national  Conference  on M  achine L  earning,  pages
 Percy  Liang.  20 1 6 .  Squad:   1 00,000+  questions  for

1 1 8 8– 1 1 96 .
 machine  comprehension  of  text.  In  Proceedings  of

the  201 6  Conference  on  Empirical M  ethods  in N  at

Hector  J  Levesque,  Ernest  Davis,  and  Leora  Morgen –

ural L  anguage  Processing,  pages  23 83 2392.

stern.  20 1 1 .  The  winograd  schema  challenge.  In

Aaai  spring  symposium: L  ogical f  ormalizations  of
 Minj oon  Seo,  Aniruddha  Kembhavi,  Ali  Farhadi,  and

commonsense  reasoning,  volume  46,  page  47 .
 Hannaneh  Hajishirzi.  20 1 7 .  Bidirectional  attention

flow  for  machine  comprehension.  In  ICLR.

Laj anugen  Logeswaran  and  Honglak  Lee.  20 1 8 .  An

efficient  framework  for  learning  sentence  represen Richard  Socher,  Alex  Perelygin,  Jean  Wu,  Jason

tations .  In  International  Conference  on L  earning
 Chuang,  Christopher  D  Manning,  Andrew  Ng,  and

Representations.
 Christopher  Potts .  20 1 3 .  Recursive  deep  models

for  semantic  compositionality  over  a  sentiment  tree

Bryan  McCann,  James  Bradbury,  Caiming  Xiong,  and

 bank.  In  Proceedings  of  the  201 3  conference  on

Richard  Socher.  20 1 7 .  Learned  in  translation:  Con

empirical  methods  in  natural  language p  rocessing,

textualized  word  vectors .  In  NIPS.
 –

pages   1 63 1 1 642 .

Oren  Melamud,  Jacob  Goldberger,  and  Ido  Dagan.

 Fu  Sun,  Linyang  Li,  Xipeng  Qiu,  and  Yang  Liu.

20 1 6.  context2vec :  Learning  generic  context  em

20 1 8 .  U-net:  Machine  reading  comprehension

bedding  with  bidirectional  LSTM.  In  CoNLL.

with  unanswerable  questions .  arXiv p  reprint

Tomas  Mikolov,  Ilya  Sutskever,  Kai  Chen,  Greg  S  Cor arXiv: 1 81 0. 06638.

rado,  and  Jeff  Dean.  20 1 3 .  Distributed  representa

tions  of  words  and  phrases  and  their  compositional Wilson  L  Taylor.   1 95 3 .  Cloze  procedure :  A  new

ity.  In  Advances  in N  eural I  nformation  Processing
 tool  for  measuring  readability.  Journalism B  ulletin,

Systems  26,  pages  3 1 1 1 –3 1 1 9 .  Curran  Associates ,
 30(4) : 4 1 5–43 3 .

Inc .

Erik  F  Tj ong  Kim  Sang  and  Fien  De  Meulder.

Andriy  Mnih  and  Geoffrey  E  Hinton.  2009 .  A  scal 2003 .  Introduction  to  the  conll-2003  shared  task:

able  hierarchical  distributed  language  model.  In
 Language-independent  named  entity  recognition.  In

D .  Koller,  D .  Schuurmans,  Y.  Bengio,  and  L.  B ot CoNLL.

tou,  editors,  Advances  in N  eural I  nformation  Pro

–  Joseph  Turian,  Lev  Ratinov,  and  Yoshua  Bengio.  20 1 0.

cessing  Systems  21 ,  pages   1 08 1 1 08 8 .  Curran  As

Word  representations :  A  simple  and  general  method

sociates ,  Inc .
 -

for  semi supervised  learning .  In  Proceedings  of  the

Ankur  P  Parikh,  Oscar  Ta¨ ckstr o¨
m,  Dipanj an  Das,  and  48th A  nnual M  eeting  of  the A  ssociation f  or  Compu

Jakob  Uszkoreit.  20 1 6 .  A  decomposable  attention
 tational L  inguistics,  ACL  ’ 1 0,  pages  3 84–394.

model  for  natural  language  inference.  In  EMNLP.

Ashish  Vaswani,  Noam  Shazeer,  Niki  Parmar,  Jakob

Jeffrey  Pennington,  Richard  Socher,  and  Christo Uszkoreit,  Llion  Jones,  Aidan  N  Gomez,  Lukasz

pher  D .  Manning .  20 1 4 .  Glove :  Global  vectors  for
 Kaiser,  and  Illia  Polosukhin.  20 1 7 .  Attention  is  all

word  representation.  In  Empirical M  ethods  in N  at you  need.  In  Advances  in N  eural I  nformation  Pro

ural L  anguage  Processing  (EMNLP),  pages   1 532–
 cessing  Systems,  pages  6000–60 1 0.

1 5 4 3 .

Pascal  Vincent,  Hugo  Larochelle,  Yoshua  Bengio,  and

Matthew  Peters,  Waleed  Ammar,  Chandra  Bhagavat Pierre-Antoine  Manzagol.  2008 .  Extracting  and

ula,  and  Russell  Power.  20 1 7 .  Semi-supervised  se composing  robust  features  with  denoising  autoen

quence  tagging  with  bidirectional  language  models .
 coders .  In  Proceedings  of  the  25th  international

In  ACL.
 conference  on M  achine  learning,  pages   1 096– 1 1 03 .

ACM.

Matthew  Peters,  Mark  Neumann,  Mohit  Iyyer,  Matt

Gardner,  Christopher  Clark,  Kenton  Lee,  and  Luke
 Alex  Wang,  Amanpreet  Singh,  Julian  Michael,  Fe

Zettlemoyer.  20 1 8a.  Deep  contextualized  word  rep lix  Hill,  Omer  Levy,  and  Samuel  Bowman.  20 1 8a.

resentations .  In  NAACL.
 Glue:  A  multi-task  benchmark  and  analysis  platform

for  natural  language  understanding.  In  Proceedings
 •  Additional  details  for  our  experiments  are

of  the  201 8  EMNLP  Workshop B  lackboxNLP: A  n presented  in  Appendix  B ;  and

alyzing  and I  nterpreting N  eural N  etworks f  or N  LP,

pages  3 5 3–3 55 .
 •  Additional  ablation  studies  are  presented  in

Wei  Wang,  Ming  Yan,  and  Chen  Wu.  20 1 8b.  Multi Appendix  C .

granularity  hierarchical  attention  fusion  networks
 We  present  additional  ablation  studies  for

for  reading  comprehension  and  question  answering.

 BERT  including :

In  Proceedings  of the  56th A  nnual M  eeting  of the A  s

sociation f  or  Computational L  inguistics  (Volume  1 :
 –

Effect  of  Number  of  Training  Steps ;  and

Long  Papers) .  Association  for  Computational  Lin

guistics .
 –  Ablation  for  Different  Masking  Proce

dures .

Alex  Warstadt,  Amanpreet  Singh,  and  Samuel  R  Bow

man.  20 1 8 .  Neural  network  acceptability j  udg A  Additional  Details  for  BERT

ments .  arXiv p  reprint  arXiv: 1 805. 1 2471 .

A. 1  Illustration  of  the  Pre-training  Tasks

Adina  Williams,  Nikita  Nangia,  and  Samuel  R  Bow

man.  20 1 8 .  A  broad-coverage  challenge  corpus
 We  provide  examples  of  the  pre-training  tasks  in

for  sentence  understanding  through  inference.  In
 the  following .

NAACL.

Masked  LM  and  the  Masking  Procedure  As

Yonghui  Wu,  Mike  Schuster,  Zhifeng  Chen,  Quoc  V
 sumin  the  unlabeled  sentence  is  my  do g  i s

g

Le,  Mohammad  Norouzi,  Wolfgang  Macherey,

ha i ry,  and  during  the  random  masking  procedure

Maxim  Krikun,  Yuan  Cao,  Qin  Gao,  Klaus

Macherey,  et  al.  20 1 6.  Google’ s  neural  ma we  chose  the  4-th  token  (which  corresponding  to

chine  translation  system:  Bridging  the  gap  between
 h a i ry) ,  our  masking  procedure  can  be  further  il

human  and  machine  translation.  arXiv p  reprint
 lustrated  b

y

arXiv: 1 609. 08144.

•  80%  of  the  time :  Replace  the  word  with  the

Jason  Yosinski,  Jeff  Clune,  Yoshua  Bengio,  and  Hod

Lipson.  20 1 4 .  How  transferable  are  features  in  deep
 [ MA S K ]   token,  e . g . ,  my  do g  i s  h a i r y  →

neural  networks ?  In  Advances  in  neural  information
 my  do g  i s   [ MAS K ]

processing  systems,  pages  3 320–3 328 .

•  1 0%  of  the  time :  Replace  the  word  with  a

Adams  Wei  Yu,  David  Dohan,  Minh-Thang  Luong,  Rui
 random  word,  e.g . ,  my  do g  i s  h a i ry  →  my

Zhao,  Kai  Chen,  Mohammad  Norouzi,  and  Quoc  V

do g  i s  app l e

Le.  20 1 8 .  QANet:  Combining  local  convolution

with  global  self-attention  for  reading  comprehen

•  1 0%  of  the  time :  Keep  the  word  un

sion.  In  ICLR .

changed,  e . g . ,  my  do g  i s  h a i r y  →  my  do g

Rowan  Zellers,  Yonatan  Bisk,  Roy  Schwartz,  and  Yej in
 i s  h a i r y .  The  purpose  of  this  is  to  bias  the

Choi.  20 1 8 .  Swag :  A  large-scale  adversarial  dataset
 re resentation  towards  the  actual  observed

for  grounded  commonsense  inference.  In  Proceed p

word.

ings  of  the  201 8  Conference  on  Empirical M  ethods

in N  atural L  anguage  Processing  (EMNLP) .

The  advantage  of  this  procedure  is  that  the

Yukun  Zhu,  Ryan  Kiros,  Rich  Zemel,  Ruslan  Salakhut Transformer  encoder  does  not  know  which  words

dinov,  Raquel  Urtasun,  Antonio  Torralba,  and  S anj a
 it  will  be  asked  to  predict  or  which  have  been  re

Fidler.  20 1 5 .  Aligning  books  and  movies :  Towards

placed  by  random  words ,  so  it  is  forced  to  keep

story-like  visual  explanations  by  watching  movies

and  reading  books .  In  Proceedings  of  the I  EEE
 a  distributional  contextual  representation  of  ev

international  conference  on  computer  vision,  pages
 ery  input  token.  Additionally,  because  random

1 9–27 .
 replacement  only  occurs  for   1 . 5 %  of  all  tokens

(i . e . ,   1 0%  of   1 5 % ) ,  this  does  not  seem  to  harm

Appendix  for  “BERT:  Pre-training  of
 the  model’ s  language  understanding  capability.  In

Deep  Bidirectional  Transformers  for
 Section  C.2,  we  evaluate  the  impact  this  proce

# ”

Language  Understanding 
 dure.

We  organize  the  appendix  into  three  sections :
 Compared  to  standard  langauge  model  training,

the  masked  LM  only  make  predictions  on   1 5 %  of

•  Additional  implementation  details  for  BERT
 tokens  in  each  batch,  which  suggests  that  more

are  presented  in  Appendix  A;
 pre-training  steps  may  be  required  for  the  model

# B E RT (Ou rs) 
 OpenAI G PT 
 E LMo

T 1   T2   . . . 
 T N 
  T 1   T2 
 . . . 
  T N 
  T 1   T2 
 . . . 
  T N

T rm   T rm   . . . 
 T rm 
 T rm   T rm   . . . 
 T rm

## Lstm 
 Lstm   . . . 
 Lstm 
 Lstm   Lstm   . . . 
 Lstm

## T rm   T rm   . . . 
 T rm 
 T rm   T rm   . . . 
 T rm 
 Lstm   Lstm   . . . 
 Lstm 
 Lstm   Lstm   . . . 
 Lstm

E 1   E 2 
 . . .     E N   
   E 1   E 2 
 . . . 
   E N 
   E 1   E 2 
 . . . 
   E N

Figure  3 :  Differences  in  pre-training  model  architectures .  BERT  uses  a  bidirectional  Transformer.  OpenAI  GPT

uses  a  left-to-right  Transformer.  ELMo  uses  the  concatenation  of  independently  trained  left-to-right  and  right-to

left  LSTMs  to  generate  features  for  downstream  tasks .  Among  the  three,  only  BERT  representations  are j  ointly

conditioned  on  both  left  and  right  context  in  all  layers .  In  addition  to  the  architecture  differences ,  BERT  and

OpenAI  GPT  are  fine-tuning  approaches,  while  ELMo  is  a  feature-based  approach.

to  converge.  In  Section  C . 1   we  demonstrate  that
 epochs  over  the  3 . 3  billion  word  corpus .  We

MLM  does  converge  marginally  slower  than  a  left use  Adam  with  learning  rate  of   1 e-4,  β1  =  0 . 9 ,

to-right  model  (which  predicts  every  token) ,  but
 β2  =  0 . 999 ,  L2  weight  decay  of  0 . 0 1 ,  learning

the  empirical  improvements  of  the  MLM  model
 rate  warmup  over  the  first   1 0,000  steps,  and  linear

far  outweigh  the  increased  training  cost.
 decay  of  the  learning  rate.  We  use  a  dropout  prob

Next  Sentence  Prediction  The  next  sentence

prediction  task  can  be  illustrated  in  the  following

examples .

Input  =  [ C L S ]  t h e  ma n  we nt  t o   [ MAS K ]  s t o r e   [ S E P ]

ability  of  0 . 1  on  all  layers .  We  use  a  g e l u  acti

vation  (Hendrycks  and  Gimpel,  20 1 6)  rather  than

the  standard  r e l u ,  following  OpenAI  GPT.  The

training  loss  is  the  sum  of  the  mean  masked  LM

likelihood  and  the  mean  next  sentence  prediction

likelihood.

he  bought  a  ga l l on   [ MASK ]  mi l k   [ SEP ] 
 Training  of  BERTBAS E  was  performed  on  4

Label  =  I sNext
 Cloud  TPUs  in  Pod  configuration  ( 1 6  TPU  chips

total) . 1 3  Training  of  BERTLARGE  was  performed

= on   1 6  Cloud  TPUs  (64  TPU  chips  total) .  Each  pre

Input    [ C L S ]  t h e  ma n   [ MAS K ]  t o  t h e   s t o r e   [ S E P ]

training  took  4  days  to  complete.

p e n gu i n   [ MAS K ]  a r e  f l i ght  # # l e s s  b i r d s   [ S E P ] 
 

Longer  sequences  are  disproportionately  expen

Label  =  NotNext
 sive  because  attention  is  quadratic  to  the  sequence

length.  To  speed  up  pretraing  in  our  experiments ,

A.2  Pre-training  Procedure
 we  pre-train  the  model  with  sequence  length  of

To  generate  each  training  input  sequence,  we  sam 1 28  for  90%  of  the  steps .  Then,  we  train  the  rest

ple  two  spans  of  text  from  the  corpus ,  which  we
 1 0%  of  the  steps  of  sequence  of  5 1 2  to  learn  the

refer  to  as  “sentences”  even  though  they  are  typ positional  embeddings .

ically  much  longer  than  single  sentences  (but  can

be  shorter  also) .  The  first  sentence  receives  the  A
 A.3  Fine-tuning  Procedure

embedding  and  the  second  receives  the  B  embed For  fine-tuning,  most  model  hyperparameters  are

ding .  50%  of  the  time  B  is  the  actual  next  sentence
 the  same  as  in  pre-training,  with  the  exception  of

that  follows  A  and  50%  of  the  time  it  is  a  random
 the  batch  size,  learning  rate,  and  number  of  train

sentence,  which  is  done  for  the  “next  sentence  pre ing  epochs .  The  dropout  probability  was  always

diction”  task.  They  are  sampled  such  that  the  com kept  at  0. 1 .  The  optimal  hyperparameter  values

bined  length  is  ≤  5 1 2  tokens .  The  LM  masking  is
 are  task- specific,  but  we  found  the  following  range

applied  after  WordPiece  tokenization  with  a  uni of  possible  values  to  work  well  across  all  tasks :

form  masking  rate  of   1 5 % ,  and  no  special  consid

eration  given  to  partial  word  pieces .
 •  Batch  size :   1 6 ,  3 2

We  train  with  batch  size  of  25 6  sequences  (25 6
 13

* https ://cloudplatform.googleblog.com/20 1 8/06/Cloud

sequences    5 1 2  tokens  =   1 28 ,000  tokens/batch)
 TPU-now-offers-preemptible-pricing-and-global

for   1 ,000,000  steps ,  which  is  approximately  40
 availability.html

•  Learning  rate  (Adam) :  5e-5 ,  3e-5 ,  2e-5
 To  isolate  the  effect  of  these  differences ,  we  per

•  Number  of  epochs :  2,  3 ,  4
 form  ablation  experiments  in  Section  5 . 1   which

demonstrate  that  the  maj ority  of  the  improvements

We  also  observed  that  large  data  sets  (e. g . ,
 are  in  fact  coming  from  the  two  pre-training  tasks

1 00k+  labeled  training  examples)  were  far  less
 and  the  bidirectionality  they  enable.

sensitive  to  hyperparameter  choice  than  small  data

sets .  Fine-tuning  is  typically  very  fast,  so  it  is  rea A.5  Illustrations  of  Fine-tuning  on  Different

sonable  to  simply  run  an  exhaustive  search  over
 Tasks

the  above  parameters  and  choose  the  model  that
 The  illustration  of  fine-tuning  BERT  on  different

performs  best  on  the  development  set.
 tasks  can  be  seen  in  Figure  4 .  Our  task- specific

models  are  formed  by  incorporating  BERT  with

A.4  Comparison  of  BERT,  ELMo  ,and
 

one  additional  output  layer,  so  a  minimal  num

OpenAI  GPT

ber  of  parameters  need  to  be  learned  from  scratch.

Here  we  studies  the  differences  in  recent  popular
 Among  the  tasks ,  (a)  and  (b)  are  sequence-level

representation  learning  models  including  ELMo,
 tasks  while  (c)  and  (d)  are  token-level  tasks .  In

OpenAI  GPT  and  BERT.  The  comparisons  be the  figure,  E  represents  the  input  embedding,  Ti

tween  the  model  architectures  are  shown  visually
 represents  the  contextual  representation  of  token  i ,

in  Figure  3 .  Note  that  in  addition  to  the  architec [ C L S ]  is  the  special  symbol  for  classification  out

ture  differences,  BERT  and  OpenAI  GPT  are  fine put,  and  [ S EP]  is  the  special  symbol  to  separate

tuning  approaches,  while  ELMo  is  a  feature-based
 non-consecutive  token  sequences .

approach.

The  most  comparable  existing  pre-training
 B  Detailed  Experimental  Setup

method  to  BERT  is  OpenAI  GPT,  which  trains  a
 B.1  Detailed  Descriptions  for  the  GLUE

left-to-right  Transformer  LM  on  a  large  text  cor Benchmark  Experiments.

pus .  In  fact,  many  of  the  design  decisions  in  BERT

Our  GLUE  results  in  Table 1   are  obtained

were  intentionally  made  to  make  it  as  close  to

from  ht t p s : / / g l u eb e n chma r k . c om/

GPT  as  possible  so  that  the  two  methods  could  be

l e a de rb o a r d  and  ht t p s : / / b l o g .

minimally  compared.  The  core  argument  of  this
 -

-  ope n a i . c om/ l angu age un s upe rvi s e d.

work  is  that  the  bi directionality  and  the  two  pre

The  GLUE  benchmark  includes  the  following

training  tasks  presented  in  Section  3 . 1   account  for

datasets ,  the  descriptions  of  which  were  originally

the  maj ority  of  the  empirical  improvements,  but

summarized  in  Wang  et  al.  (20 1 8a) :

we  do  note  that  there  are  several  other  differences

between  how  BERT  and  GPT  were  trained:
 MNLI  Multi-Genre  Natural  Language  Inference

is  a  large-scale,  crowdsourced  entailment  classifi

•  GPT  is  trained  on  the  B ooksCorpus  (800M
 cation  task  (Williams  et  al. ,  20 1 8) .  Given  a  pair  of

words) ;  BERT  is  trained  on  the  B ooksCor sentences,  the  goal  is  to  predict  whether  the  sec

pus  (800M  words)  and  Wikipedia  (2,500M
 ond  sentence  is  an  entailment,  contradiction,  or

words) .
 neutral  with  respect  to  the  first  one .

•  GPT  uses  a  sentence  separator  ( [ S E P ] )  and
 QQP  Quora  Question  Pairs  is  a  binary  classifi

classifier  token  ( [ C L S ] )  which  are  only  in cation  task  where  the  goal  is  to  determine  if  two

troduced  at  fine-tuning  time;  BERT  learns
 questions  asked  on  Quora  are  semantically  equiv

[ S E P ] ,  [ C L S ]   and  sentence  A/B  embed alent  (Chen  et  al. ,  20 1 8) .

dings  during  pre-training .

QNLI  Question  Natural  Language  Inference  is

•  GPT  was  trained  for   1 M  steps  with  a  batch
 a  version  of  the  Stanford  Question  Answering

size  of  3 2,000  words ;  BERT  was  trained  for
 Dataset  (Rajpurkar  et  al. ,  20 1 6)  which  has  been

1 M  steps  with  a  batch  size  of   1 28 ,000  words .
 converted  to  a  binary  classification  task  (Wang

et  al. ,  20 1 8a) .  The  positive  examples  are  (ques

•  GPT  used  the  same  learning  rate  of  5e-5  for
 tion,  sentence)  pairs  which  do  contain  the  correct

all  fine-tuning  experiments ;  BERT  chooses  a
 answer,  and  the  negative  examples  are  (question,

task-specific  fine-tuning  learning  rate  which
 sentence)  from  the  same  paragraph  which  do  not

performs  the  best  on  the  development  set.
 contain  the  answer.

### C l ass  
 C l ass

### La be l 
 La be l

C   T 1   . . .   T N 
 T [S E P]   T 1 ’   
 . . .   T M ’ 
 
 C   T 1 
  T2 
 . . . 
  T N

# B E RT 
 B E RT

E [C LS  ]   E 1 
 . . .   E N 
   E [S E P] 
 E 1 ’   
 . . .   E M ’ 
 
 E [C LS]   E 1 
   E 2 
 . . .     E N

[C LS]   To1 k  
 . . . TNo k  

  [S E P]   To1 k  
 
 . . .   TMo k 
 
 [C L S]   To k  1   To k 2   . . .   To k  N

## Se nte n ce  1 
 Se nte n ce 2 
 S i ng l e Se nte n ce

### Sta rt/E n d S pa n 
 O   B- P E R   . . . 
 O

C   T 1   . . .   T N 
 T [S E P]   T 1 ’   
 . . .   T M ’ 
 
 C   T 1 
  T2 
 . . . 
  T N

# B E RT 
 B E RT

E [C LS]   E 1 
 . . .   E N 
   E [S E P]   E 1 ’   
 . . .   E M ’ 
 
 E [C LS]   E 1 
   E 2 
 . . . 
   E N

[C LS]   To1 k  
 
 . . . TNo k  

  [S E P]   To1 k  
 
 . . .   TMo k 
 
 [C L S]   To k  1   To k 2   . . .   To k  N

## Qu estion   Pa rag ra p h 
 S i ng l e Se nte n ce

Figure  4 :  Illustrations  of  Fine-tuning  BERT  on  Different  Tasks .

SST-2  The  Stanford  Sentiment  Treebank  is  a
 for  whether  the  sentences  in  the  pair  are  semanti

binary  single- sentence  classification  task  consist cally  equivalent  (Dolan  and  Brockett,  2005) .

ing  of  sentences  extracted  from  movie  reviews

with  human  annotations  of  their  sentiment  (Socher
 RTE  Recognizing  Textual  Entailment  is  a  bi

et  al .   20 1 3 .
 nary  entailment  task  similar  to  MNLI,  but  with

, ) 1 4

much  less  training  data  (B entivogli  et  al. ,  2009) .

CoLA  The  Corpus  of  Linguistic  Acceptability  is

a  binary  single-sentence  classification  task,  where
 WNLI  Winograd  NLI  is  a  small  natural  lan

the  goal  is  to  predict  whether  an  English  sentence
 guage  inference  dataset  (Levesque  et  al. ,  20 1 1 ) .

is  linguistically  “acceptable”  or  not  (Warstadt
 The  GLUE  webpage  notes  that  there  are  issues

et  al . ,  20 1 8) .
 with  the  construction  of  this  dataset,  1 5  and  every

’

trained  system  that s  been  submitted  to  GLUE  has

STS-B  The  Semantic  Textual  Similarity  Bench performed  worse  than  the  65 . 1  baseline  accuracy

mark  is  a  collection  of  sentence  pairs  drawn  from
 of  predicting  the  maj ority  class .  We  therefore  ex

news  headlines  and  other  sources  (Cer  et  al. ,
 clude  this  set  to  be  fair  to  OpenAI  GPT.  For  our

20 1 7) .  They  were  annotated  with  a  score  from   1 
 GLUE  submission,  we  always  predicted  the  ma

to  5  denoting  how  similar  the  two  sentences  are  in

terms  of  semantic  meaning .
 14Note  that  we  only  report  single-task  fine-tuning  results

in  this  paper.  A  multitask  fine-tuning  approach  could  poten

MRPC  Microsoft  Research  Para hrase  Cor us
 tially  push  the  performance  even  further.  For  example,  we

p p did  observe  substantial  improvements  on  RTE  from  multi

consists  of  sentence  pairs  automatically  extracted
 task  training  with  MNLI.

from  online  news  sources,  with  human  annotations
 15 ht tp s : / / g l uebe n chma rk . c om/ f aq

j ority  clas s .
 Note  that  the  purpose  of  the  masking  strategies

is  to  reduce  the  mismatch  between  pre-training

C  Additional  Ablation  Studies
 and  fine-tuning,  as  the  [ MAS K ]   symbol  never  ap

C.1  Effect  of  Number  of  Training  Steps

pears  during  the  fine-tuning  stage.  We  report  the

Dev  results  for  both  MNLI  and  NER.  For  NER,

Figure  5  presents  MNLI  Dev  accuracy  after  fine we  report  both  fine-tuning  and  feature-based  ap

tuning  from  a  checkpoint  that  has  been  pre-trained
 proaches,  as  we  expect  the  mismatch  will  be  am

for  k  steps .  This  allows  us  to  answer  the  following
 plified  for  the  feature-based  approach  as  the  model

questions :
 will  not  have  the  chance  to  adjust  the  representa

tions .

1 .  Question:  Does  BERT  really  need  such

a  large  amount  of  pre-training  ( 1 28 ,000
 Masking  Rates  Dev  Set  Results

words/batch  *   1 ,000,000  steps)  to  achieve
 MAS K  S AME  RND  MNLI  NER

hi h  fine-tunin  accurac ?
 Fine-tune  Fine-tune  Feature-based

g g y

Answer:  Yes ,  BERTBASE  achieves  almost
 80%  1 0%  1 0%  84.2  95 .4  94.9

1 00%  0%  0%  84 . 3  94 . 9  94 .0

1 .0%  additional  accuracy  on  MNLI  when
 80%  0%  20%  84. 1  95 .2  94.6

trained  on   1 M  steps  compared  to  500k  steps .
 80%  20%  0%  84.4  95 .2  94.7

0%  20%  80%  8 3 .7  94 . 8  94 . 6

2 .  Question :  Does  MLM  pre-training  converge
 0%  0%  1 00%  83 .6  94.9  94.6

slower  than  LTR  pre-training,  since  only   1 5 %

of  words  are  predicted  in  each  batch  rather
 Table  8 :  Ablation  over  different  masking  strategies .

than  every  word?

Answer:  The  MLM  model  does  converge
 The  results  are  presented  in  Table  8 .  In  the  table,

slightly  slower  than  the  LTR  model.  How MAS K  means  that  we  replace  the  target  token  with

ever,  in  terms  of  absolute  accuracy  the  MLM
 the  [ MAS K ]   symbol  for  MLM ;  S AME  means  that

model  begins  to  outperform  the  LTR  model
 we  keep  the  target  token  as  is ;  RND  means  that

almost  immediately.
 we  replace  the  target  token  with  another  random

token .

C.2  Ablation  for  Different  Masking
 The  numbers  in  the  left  part  of  the  table  repre

Procedures
 sent  the  probabilities  of  the  specific  strategies  used

during  MLM  pre-training  (BERT  uses  80% ,   1 0% ,

In  Section  3 . 1 ,  we  mention  that  BERT  uses  a

1 0%) .  The  right  part  of  the  paper  represents  the

mixed  strategy  for  masking  the  target  tokens  when

- Dev  set  results .  For  the  feature-based  approach,

pre training  with  the  masked  language  model

we  concatenate  the  last  4  layers  of  BERT  as  the

(MLM)  obj ective.  The  following  is  an  ablation

features,  which  was  shown  to  be  the  best  approach

study  to  evaluate  the  effect  of  different  masking

in  S ection  5 . 3 .

strategies .

From  the  table  it  can  be  seen  that  fine-tuning  is

surprisingly  robust  to  different  masking  strategies .

84
 However,  as  expected,  using  only  the  MAS K  strat

y 
 egy  was  problematic  when  applying  the  feature

car 8 2

cu based  approach  to  NER.  Interestingly,  using  only

c

Av  80
 the  RND  strategy  performs  much  worse  than  our

e

DI  strategy  as  well.

L

N 78

M

BERTBASE  (Masked  LM)

76
 BERTBASE  (Left-to-Right)

2 0 0  40 0  6 0 0  8 0 0  1 , 0 0 0

Pre-training  Steps  (Thousands)

Figure  5 :  Ablation  over  number  of  training  steps .  This

shows  the  MNLI  accuracy  after  fine-tuning,  starting

from  model  parameters  that  have  been  pre-trained  for

k  steps .  The  x- axis  is  the  value  of  k .
