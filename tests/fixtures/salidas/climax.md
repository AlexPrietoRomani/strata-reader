2023- 1 2- 1 9

###### C l i m aX :

## A  fou ndation  model  for  weather  a nd  cl i mate

Tu n g   N gu ye n 1
 ,  J o h a n n es  B ra n d stette r2
 ,  As h i s h   Ka p oor3
 ,

∗ 2
 ∗ 1

###### J ayesh  K .  G u pta ,  a n d  Ad itya  G rover

1 U C LA ,   2 M i c rosoft ,   3 Sca l ed  Fo u n d a t i o n s

Most  state-of-the-art  approaches  for  weather  and  climate  modeling  are  based  on  physics-informed

## 3

numerical  models  of  the  atmosphere.  These  approaches  aim  to  model  the  non-linear  dynamics

## 2

0 and  complex  interactions  between  multiple  variables,  which  are  challenging  to  approximate.

2  Additionally,  many  such  numerical  models  are  computationally  intensive,  especially  when

c modeling  the  atmospheric  phenomenon  at  a  fine-grained  spatial  and  temporal  resolution.

e Recent  data-driven  approaches  based  on  machine  learning  instead  aim  to  directly  solve  a

D  downstream  forecasting  or  projection  task  by  learning  a  data-driven  functional  mapping  using

8 deep  neural  networks.  However,  these  networks  are  trained  using  curated  and  homogeneous

## 1

climate  datasets  for  specific  spatiotemporal  tasks ,  and  thus  lack  the  generality  of  numerical

] models.  We  develop  and  demonstrate  ClimaX,  a  flexible  and  generalizable  deep  learning

G model  for  weather  and  climate  science  that  can  be  trained  using  heterogeneous  datasets

L. spanning  different  variables,  spatio-temporal  coverage,  and  physical  groundings.  ClimaX

s extends  the  Transformer  architecture  with  novel  encoding  and  aggregation  blocks  that  allow

## c

[ effective  use  of  available  compute  while  maintaining  general  utility.  ClimaX  is  pre-trained

with  a  self-supervised  learning  objective  on  climate  datasets  derived  from  CMIP6 .  The  pre

## 5

trained  ClimaX  can  then  be  fine-tuned  to  address  a  breadth  of  climate  and  weather  tasks,

## v

3 including  those  that  involve  atmospheric  variables  and  spatio-temporal  scales  unseen  during

4 pretraining.  Compared  to  existing  data-driven  baselines,  we  show  that  this  generality  in  ClimaX

## 3

results  in  superior  performance  on  benchmarks  for  weather  forecasting  and  climate  projections,

## 0

1 even  when  pretrained  at  lower  resolutions  and  compute  budgets.  Source  code  is  available  at

## .

1 https : //github . com/mi cro soft /Cl imaX .

## 0

## 3

### 2: Cl i mate 
 Spatia l

# v C l i m a X 
 Downscaling

### i Proj ecti o ns

###### X Down sca l i n g 
 Reg i o n a l

## r

## a

#### G l o b a l

### Te m po ra l

Δ�� ≈  h rs    Δ�� ≈  d a  ys   Δ�� ≈  we  e ks   Δ�� ≈  m o  nt h s/ye a r

###### N owca sti n g   S h o rt & M e d i u m - ra n g e   S u b-se a so n a l   S e a so n a l

F igu re  1 :  ClimaX  is  built  as  a  foundation  model  for  any  weather  and  climate  modeling  task .  On  the  weather

front ,  these  tasks  include  standard  forecasting  tasks  for  various  lead-time  horizons  at  various  resolutions ,  both

globally  or  regionally.  On  the  climate  front ,  making  long  term  proj ections  and  obtaining  downscaling  results

from  lower  resolution  model  outputs  are  standard  tasks .

∗ Equal  contributions  as  last  authors ,  listed  reverse  alphabetically

Author  email (s) :  tungnd@cs . ucla. edu , j  ohannesb@microsoft . com ,  ashish . kapoor@gmail . com ,

j kg@cs . stanford . edu ,  adityag@cs . ucla. edu

ClimaX :  A  foundation  model  for  weather  and  climate

# Co nte nts

1  Introduct ion  4

2  Background  and  Related  Work  5

2 . 1   D at a   s o u r c e s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   6

2 . 1 . 1   C M I P 6   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   6

2 . 1 . 2   E R A 5   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   6

2 . 2   Ta s k s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   7

2 . 3   Fo u n d at i o n   m o d e l s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   8

3  Approach  8

3 . 1  I n p u t  r e p r e s e nt at i o n   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   8

3 . 2   M o d e l   a r c h i t e c t u r e   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   8

3 . 2 . 1  Var i ab l e  t o ke n i z at i o n   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   9

3 . 2 . 2  Var i ab l e   ag g r e g at i o n   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   9

3 . 2 . 3   Tr a n s fo r m e r   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 0

3 . 3   D at a s e t s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 0

3 . 3 . 1   P r e t r a i n i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 0

3 . 3 . 2  F i n e t u n i n g  an d  e val u at i o n   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 1

3 . 4   Tr a i n i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 1

3 . 4 . 1   P r e t r a i n i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 1

3 . 4 . 2   F i n e t u n i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 1

4  Experiments  1 2

4 . 1   N e u r a l  b a s e l i n e s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 2

4 . 2   Fo r e c a s t i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 2

4 . 2 . 1   G l o b a l  fo r e c a s t i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 2

4 . 2 . 2   Re g i o n a l  fo r e c a s t i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 3

4 . 2 . 3  S ub- s e as o n al  t o  s e as o n al  c umul at ive  p re d i ct i o n   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 4

4 . 3   C l i m at e  p r o j e c t i o n   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 4

4 . 4   C l i m at e  m o d e l  d ow n s c al i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 6

4 . 5   S c a l i n g   l aw s   a n a l y s i s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 7

4 . 6   A b l at i o n   s t u d i e s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 8

4 . 6 . 1  S hould  we  finet une  C limaX  for  each  variable  separat ely  or  all  at  once ?   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 8

4 . 6 . 2  S ho uld  we  d o  it er at ive  fore c ast  or  d i re ct  fore c ast ?   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 8

4 . 6 . 3  C an  we  fi net une  C li m aX  t o  work  for  all  le ad  t i mes ?   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   1 9

5  Discussion  and  Future  Work  1 9

Acknowledgments  2 1

2

ClimaX :  A  foundation  model  for  weather  and  climate

A  Model  29

A . 1   C l i m a X   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   2 9

A . 1 . 1  I m p l e m e nt at i o n  d e t a i l s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   2 9

A . 1 . 2   H y p e r p ar a m e t e r s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   2 9

A . 2   C N N   B a s e l i n e s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 0

A . 2 . 1  Re s N e t  H y p e r p ar am e t e r s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 0

A . 2 . 2  U N e t  H y p e r p ar am e t e r s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 0

A . 2 . 3   O t h e r  i m p l e m e nt at i o n  d e t ai l s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 0

B  Training  details  30

B . 1   P r e t r a i n i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 1

B . 1 . 1   O b j e c t i ve   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 1

B . 1 . 2   O p t i m i z at i o n   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 1

B . 2   F i n e t u n i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 1

B . 2 . 1   O b j e c t i ve   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 1

B . 2 . 2   O p t i m i z at i o n   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 1

C  Datasets  3 1

C . 1   C M I P 6 - C l i m aX   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 1

C . 2   E R A 5   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 2

C . 2 . 1   E R A 5 - N A   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 2

C . 2 . 2   E R A - S 2 S   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 2

C . 3   C l i m at e B e n c h   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 2

D  Quantitative  evaluation  33

D . 1   M e t r i c s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 3

D . 1 . 1  We at h e r  fo r e c as t i n g  m e t r i c s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 4

D . 1 . 2   C l i m at e  p r o j e c t i o n  m e t r i c s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 4

D . 1 . 3   C l i m at e  d ow n s c al i n g  m e t r i c s   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 5

D . 2   Re s u l t s  s u m m a r y   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 5

E  Qualitat ive  evaluat ion  38

E . 1   N owc a s t i n g   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 8

E . 2  S ho rt  and  me d i u m- r ange  we at he r  fo re c ast i ng   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   3 9

E . 3  L o nge r  h o r i z o n  i n s t ant ane o u s  fo r e c as t i ng   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .   4 1

3

ClimaX :  A  foundation  model  for  weather  and  climate

1 .   I n t ro d u ct i o n

Modeling  weather  and  climate  is  an  omnipresent  challenge  for  science  and  society.  With  rising  concerns  around

extreme  weather  events  and  climate  change ,  there  is  a  growing  need  for  both  improved  weather  forecasts

for  disaster  mitigation  and  climate  proj ections  for  long-term  policy  making  and  adaptation  efforts  [MD + 2 1] .

Currently,  numerical  methods  for  global  modeling  of  weather  and  climate  are  parameterized  via  various

general  circulation  models  ( GCM)  [Lyn08] .  GCMs  represent  system  of  differential  equations  relating  the  flow

of  energy  and  matter  in  the  atmosphere ,  land ,  and  ocean  that  can  be  integrated  over  time  to  obtain  forecasts

for  relevant  atmospheric  variables  [Lyn08 ;  BTB 1 5] .  While  extremely  useful  in  practice ,  GCMs  also  suffer  from

many  challenges ,  such  as  accurately  representing  physical  processes  and  initial  conditions  at  fine  resolutions ,

as  well  as  technological  challenges  in  large-scale  data  assimilation  and  computational  simulations   [Bau+ 20] .

These  factors  limit  their  use  in  many  scenarios ,  especially  in  simulating  atmospheric  variables  quickly  at  very

short  time  scales   (e . g . ,  a  few  hours)  or  accurately  at  long  time  scales   (e . g . ,  beyond  5- 7  days)   [Zha+ 1 9] .

In  contrast ,  there  has  been  a  steady  rise  in  data-driven  approaches  for  forecasting  of  atmospheric  variables ,

especially  for  meteorological  applications  [GKH 1 5 ;  DB 1 8 ;  Web+ 20 ;  SM 1 9 ;  Sch 1 8 ;  Kas+ 2 1 ;  Sch+ 2 1 ;  Rei+ 1 9 ;

Hun+ 1 9 ;  Sch+ 1 7] .  The  key  idea  here  is  to  train  deep  neural  networks  to  predict  the  target  atmospheric

variables  using  decades  of  historical  global  datasets ,  such  as  the  ERA- 5  reanalysis  dataset   [Her+ 20] .  Unlike

G CMs ,  these  networks  are  not  explicitly  grounded  in  physics ,  and  lack  general-purpose  utility  for  Earth

system  sciences  as  they  are  trained  for  a  specific  predictive  modeling  task .  Yet ,  with  growing  compute  and

datasets ,  there  is  emerging  evidence  that  these  models  can  achieve  accuracies  competitive  with  state-of-the-art

numerical  models  in  many  scenarios ,  such  as  nowcasting  of  precipitation  [Rav+2 1 ;  Søn+20]  and  medium-range

forecasting  of  variables  like  temperature ,  wind  and  humidity  [WD C20 ;  RT2 1 ;  Kei22 ;  Pat +22 ;  Bi+22 ;  Lam+22] .

While  these  trends  are  encouraging,  there  remain  concerns  regarding  the  generality  of  such  data-driven

methods  to  diverse  real-world  scenarios ,  such  as  forecasting  of  extreme  weather  events  and  longer-term  climate

proj ections ,  especially  under  limited  spatiotemporal  supervision  and  computational  budgets .

Variants  of  the  aforementioned  challenges  apply  broadly  throughout  machine  learning  (ML) .  In  disciplines

such  as  natural  language  processing  and  computer  vision ,  it  is  well  acknowledged  that  ML  models  trained

to  solve  a  single  task  using  supervised  learning  are  label-hungry  during  training  and  brittle  when  deployed

outside  their  training  distribution   [Tao+ 20] .  Recent  works  have  shown  that  it  is  possible  to  mitigate  the

supervision  bottleneck  by  pretraining  [Dev+ 1 8 ;  He+22]  large  unsupervised “  foundation”  models  [Bom+2 1]

on  huge  passive  datasets ,  such  as  text  and  images  scraped  from  the  internet   [Ram+ 22 ;  Bro+ 20 ;  Liu+ 2 1 ;

Ree+ 22b] .  Post  pretraining,  there  are  many  ways  to  finetune  the  same  model  on  arbitrary  target  task (s)

with  little  to  none   (i . e . ,  zero-shot )  additional  supervision .  Besides  low  t arget  supervision ,  these  models  also

generalize  better  to  shifts  outside  their  training  distribution   [Hen+ 20a;  Zha+ 2 2b] ,  improving  their  reliability.

Inspired  by  the  above  successes ,  this  work  studies  the  question :  how  do  we  design  and  train  a  foundation

model  for  weather  and  climate  that  can  be  efficiently  adapted  for  general-purpose  tasks  concerning  the  Earth ’s

atmosphere?  We  propose  ClimaX ,  a  foundation  model  for  weather  and  climate .  For  pretraining  any  foundation

model ,  the  key  recipe  is  to  train  a  deep  architecture  on  a  large  dataset  using  an  unsupervised  obj ective .  For

example ,  many  foundation  models  for  language  and  vision  train  large  transformers  on  Internet-scale  datasets

using  generative  modeling.  While  conceptually  simple ,  this  scaling  recipe  is  riddled  with  challenges  for  weather

and  climate  domains ,  that  we  discuss  below  and  propose  to  resolve  with  ClimaX .

First ,  it  is  unclear  what  constitutes  an  Internet-scale  passive  dataset  for  pretraining  ClimaX .  The  size  of

historical  weather  and  climate  datasets  at  any  given  time  is  fixed  and  increases  at  an  almost  constant  rate

everyday,  as  it  corresponds  to  processed  sensor  measurements  of  naturally  occurring  phenomena.  Our  first  key

proposal  is  to  go  beyond  these  datasets  to  explicitly  utilize  physics-informed  climate  simulation  models .  Many

such  models  are  in  use  today,  for  example ,  the  CMIP6  collection   [Eyr+ 1 6]  of  climate  modeling  simulations

consists  of  runs  of  ∼ 1 00  distinct  climate  models  from  49  different  climate  modeling  groups .  We  show  that  the

heterogeneity  in  these  simulation  datasets  serves  as  a  source  of  rich  and  plentiful  data  for  pretraining  ClimaX .

Second ,  we  need  a  model  architecture  that  can  aptly  embrace  the  heterogeneity  of  the  above  climate  datasets .

Climate  data  is  highly  multimodal ,  as  observations  typically  correspond  to  many  different ,  unbounded  variables

4

ClimaX :  A  foundation  model  for  weather  and  climate

with  varying  datatypes  (e . g. ,  pressure ,  temperature ,  humidity) .  Moreover ,  many  observational  datasets  are

irregular  in  the  sense  that  they  differ  in  their  spatiotemporal  coverage  and  might  correspond  to  different

subsets  of  atmospheric  variables .  We  resolve  the  above  challenges  in  ClimaX  by  repurposing  the  vision

transformer   [Dos+ 20 ;  Vas+ 1 7] .  In  contrast  to  earlier  work  where  the  input  data  is  represented  as  an  image

with  different  atmospheric  variables  treated  as  the  channels  thereof   [Pat + 22 ;  Bi+ 22] ,  we  treat  them  as  separate

modalities  to  enable  more  flexible  training  even  with  irregular  datasets .  This  has  the  side-effect  of  drastically

increasing  the  sequence  length ,  which  we  propose  to  resolve  via  a  cross-attention  style  channel  aggregation

scheme  prior  to  the  self- attention  layers .

Third  and  last ,  we  need  a  pretraining  obj ective  that  can  learn  complex  relationships  between  the  atmospheric

variables  and  permit  effective  finetuning  for  downstream  tasks .  Given  the  spatiotemporal  nature  of  climate

data,  we  propose  a  randomized  forecasting  obj ective  for  pretraining  ClimaX .  Here ,  the  goal  of  the  model

is  to  forecast  an  arbitrary  set  of  input  variables  at  an  arbitrary  time  into  the  future .  While  simple  and

intuitive ,  we  show  that  such  a  pretraining  obj ective  aids  finetuning  to  novel  tasks  and  timescales  even  beyond

the  pretraining  window ,  such  as  sub-seasonal  to  seasonal  cumulative  predictions ,  climate  proj ections ,  and

downscaling  of  climate  models .  See  Figure  1  for  a  list  of  t asks  considered  in  this  work .

Empirically,  we  demonstrate  that  a  single  pretrained  model  can  be  finetuned  for  many  tasks  (e . g. ,  multi-scale

weather  forecasting,  climate  proj ections ,  downscaling)  under  a  range  of  operating  conditions  involving  different

spatiotemporal  resolutions ,  geographical  regions ,  and  target  prediction  variables ,  including  those  unseen  during

training.  Notably,  our  benchmark  results  are  state-of-the-art  on  ClimateBench  [WP +22]  and  competitive

with  the  operational  Integrated  Forecasting  System  (IFS )  [Wed+ 1 5]  on  WeatherBench  [Ras+20] ,  even  when

our  model  is  trained  on  moderate  resolutions  using  only  a  maximum  of  80  NVIDIA  V 1 00  GPUs .

Finally,  we  show  promising  scaling  laws  of  ClimaX  with  natural  axes  of  performance  improvements  for  larger

number  of  pre-training  datasets ,  larger  models ,  and  scaling  to  higher  resolution  gridded  datasets .  While

especially  the  last  is  in  line  with  recent  and  concurrent  works  on  data-driven  weather  forecasting   [Pat + 2 2 ;

Bi+ 2 2 ;  Lam+ 2 2] ,  to  the  best  of  our  knowledge ,  ClimaX  is  the  first  of  its  kind  data-driven  model  that  can

effectively  scale  using  heterogeneous  climate  datasets  during  pretraining,  and  generalize  to  diverse  downstream

tasks  during  finteuning,  paving  the  way  for  a  new  generation  of  data-driven  models  for  Earth  systems  science .

# 2 .  Ba c kgrou n d  a n d  Rel ated  Work

Current  weather  and  climate  models  in  use  today  rely  extensively  on  numerical  methods  and  computational

simulations  to  predict  and  understand  the  Earth ’s  weather  and  climate  systems .  These  tasks  include  various

numerical  weather  prediction  (NWP )  systems  which  use  computer  simulations  to  make  short-term  forecasts

of  weather  conditions  as  well  as  climate  models  which  use  similar  techniques  to  simulate  and  predict  the

long-term  changes  in  the  Earth ’ s  climate .  Most  notably,  at  the  core  of  both  weather  and  climate  models  lie

the  same  set  of  primitive  equations .

For  climate  modeling,  earth  system  models  (ESM)  [Hur+ 1 3] ,  or “  coupled  models”,  that  couple  together

simulations  which  govern  the  atmosphere ,  cryosphere ,  land ,  and  ocean  processes  are  considered  the  state-of

the-art .  Primarily  these  simulations  are  based  on  general  circulation  models  ( G CMs)   [Sat04 ;  Lyn08 ;  Ado 1 4 ;

MD + 2 1]  which  date  back  to  the  works  of  Phillips   [Phi56]  and  Lorenz   [Lor67]  solving  Navier- Stokes  equations

on  a  rotation  sphere  to  model  fluid  circulation .  These  models  are  often  used  to  perform  various  factor

s ensitivity  studies  to  examine  how  the  changes  in  certain  forcing  factors  like  greenhouse  gas  concentrations

can  affect  the  global  or  regional  climate  and  help  in  climate  projections  to  help  understand  future  conditions .

Numerical  Weather  Prediction  (NWP)  models  share  many  components  of  GCMs ,  especially  the  atmospheric

components  [BTB 1 5 ;  Lyn08 ;  Kal03] .  However ,  incorporating  data   assimilation  [LS Z 1 5 ;  Gro22]  which  involves

combining  observations  and  various  measurements  of  the  atmosphere  and  oceans  together  with  these  numerical

models  is  important  for  accurate  forecasts  and  simulations .  Another  significant  distinction  between  weather

and  climate  models  is  the  framing  of  the  solution  for  underlying  equations :  initial  value  pro blem  for  weather ,

while  boundary  value  pro blem  for  climate  [BTB 1 5] .  Different  difficulty  levels  of  these  solution  approaches  results

5

ClimaX :  A  foundation  model  for  weather  and  climate

in  the  fact  where  climate  models  tend  to  be  global  often  at  coarser  spatio-temporal  resolutions  while  weather

models  can  range  from  global  to  local  and  regional  models  of  very  high  spatio-temporal  resolutions   [War 1 0] .

Despite  their  noted  success ,  including  the  recent  202 1  Nobel  Prize  in  Physics   [RRH2 2] ,  there  is  considerable

debate  around  the  limitations  of  general  circulation  models  ( G CMs) ,  particularly  structural  errors  across

models  and  the  fact  that  current  GCMs  are  designed  to  reproduce  observed  climate  [Bal+ 22] .  The  climate

science  community  has  been  aware  of  these  challenges  which  resulted  in  the  creation  of  Coupled  Model

Intercomparison  Proj ect  ( CMIP )  as  a  standardized  protocol  for  evaluating  and  comparing  the  performance  of

different  climate  models   [Mee+00] .  As  we  will  see  in  the  following  sections ,  not  only  has  CMIP  been  playing  a

crucial  role  in  the  advancement  of  our  understanding  of  climate  change  and  its  potential  impacts ,  its  evaluation

procedure  has  resulted  in  enormous  quantity  of  data  making  modern  deep  learning  based  approaches  quite

attractive  for  many  tasks .  Notably,  encoding  this  knowledge  into  a “  foundation”  machine  learning  model  with

much  faster  inference  and  data  assimilation  capabilities  can  pave  the  way  for  a  much  wider  impact .

2 . 1 .  D ata  sou rces

Unlike  data  in  computer  vision  or  natural  language  processing,  weather  and  climate  data  is  not  solely  based  on

sensed  data,  instead  incorporates  information  from  a  diverse  range  of  sources .  For  example ,  reanalysis  weather

data  blends  meteorological  observations  with  past  short-range  weather  forecasts  via  data  assimilation   [BTB 1 5] .

The  data  measurements  themselves  are  highly  heterogeneous ,  representing  various  physical  variables  with

different  data  types  (e . g.  pressure ,  temperature ,  humidity)  that  are  recorded  at  different ,  relatively  sparse ,

spatial  locations  at  different  temporal  frequencies .  These  measurements  can  be  integrated  together  with

known  physics  inform  the  design  of  climate  simulations ,  which  again  produce  data  with  different  variables

at  different  scales .  From  a  machine  learning  perspective ,  the  plethora  of  available  data  thus  spans  multiple

axes :  from  direct  weather  measurements  at  land ,  sea,  or  atmosphere ,  over  multiple  decades  of  re-analyzed

weather  data  at  different  spatial  scales ,  to  physics-informed  climate  proj ections  for  various  scenarios .  Most

notably,  the  data  shares  the  same  set  of  primitive  equations ,  but  with  fairly  different  characteristics .  Below

we  describe  two  of  the  most  commonly  used  data  sources  for  weather  and  climate  modeling.

2 . 1 . 1 .  C M I P 6

The  Coupled  Model  Intercomparison  Proj ect  ( CMIP )  [Mee+00]  is  an  international  effort  across  different

individual  climate  modeling  groups  to  come  together  to  compare  and  evaluate  their  global  climate  models .

While  the  main  goal  of  CMIP  is  to  improve  the  understanding  of  Earth ’s  climate  system  and  improve

the  accuracy  of  its  simulations ,  the  recent  data  from  their  experimental  runs  is  easily  accessible  on  the

CMIP6   [Eyr+ 1 6]  archive .  In  CMIP6 ,  where “  6”  refers  to  the  most  recent  phase  of  the  proj ect ,  49  groups  are

involved  with  their  experiments  covering  wide  range  of  climate  variables  including  temperature ,  precipitation ,

sea  level  and  others  from  hundreds  of  models .  This  results  in  global  proj ections  of  various  climate  scenarios

from  as  early  as  1 850  onwards ,  all  following  similar  governing  equations ,  but  with  different  forcings,  e . g . ,

greenhouse  gas  emissions  that  affect  the  climate .

2 . 1 . 2 .  E R A5

The  ERA5  reanalysis  archive  [Her+ 1 8 ;  Her+20]  of  the  European  Center  for  Medium-Range  Weather  Forecasting

(ECMWF)  is  the  predominant  data  source  for  learning  and  benchmarking  weather  forecasting  systems .  Once

completed ,  the  ERA5  reanalysis  is  set  to  embody  a  detailed  record  of  the  global  atmosphere ,  land  surface

and  ocean  waves  from  1 950  onwards .  The  currently  available  ERA5  reanalysis  data  combines  the  state  of

the  art  forecasting  model  called  Integrated  Forecasting  System  (IFS )  [Wed+ 1 5]  of  ECMWF  with  available

observations  to  provide  the  best  guess  of  the  state  of  the  atmosphere ,  ocean-wave  and  land-surface  quantities

at  any  point  in  time .  In  its  raw  form ,  the  available  reanalyzed  dat a  is  huge :  40  years ,  from  1 9 79  to  20 1 8 ,  on  a

0 . 2 5 °  ×   0 . 2 5 °  global  latitude-longitude  grid  of  the  Earth ’ s  sphere ,  at  hourly  intervals  with  different  climate

variables  at  3 7  different  altitude  levels  plus  the  Earth ’ s  surface .  The  grid  overall  contains  72 1  ×   1 440  grid

points  for  latitude  and  longitude ,  respectively.  The  altitude  levels  are  presented  as  pressure  levels .

6

ClimaX :  A  foundation  model  for  weather  and  climate

2 . 2 .   Ta s ks

Given  the  scale  of  data  availability,  increasing  compute  requirements  of  current  numerical  methods  despite

it  being  difficult  to  incorporate  real  observational  data  into  them ,  machine  learning  is  increasingly  finding

applications  in  many  of  the  tasks  related  to  weather  and  climate  modeling.  When  it  comes  to  weat her ,

the  main  task  of  interest  is  forecasting  the  future  values  of  key  weather  variables .  These  tasks  can  take  the

following  forms  depending  on  temporal  and  spatial  horizons  of  interest :

•  G lobal  forecast ing  tasks  that  range  from  a  few  hours  (i . e . ,  nowcasting)  to  days  and  weeks  in  lead  time

(i . e . ,  short  and  medium  range  forecasting) .  Often  these  tasks  are  evaluated  on  the  ERA5  reanalysis  dataset

(see  Section  2 . 1 . 2)  with  Operational  IFS  [Wed+ 1 5]  of  the  European  Center  for  Medium-Range  Weather

Forecasting  (ECMWF)  being  the  current  state-of-the-art  NWP  baselines .

•  Regional  forecast ing  tasks  which  could  range  from  weather  forecasting  in  continental  North  America  or

Europe  to  individual  st ate ,  county  or  city.

•  S ub-seasonal  to  seasonal  predict ion   ( S 2 S )   [VR1 8 ;  Vit + 22]  which  is  the  task  of  forecasting  the  weather

with  lead  times  between  2  weeks  and  2  months .  S 2 S  bridges  the  gap  between  weather  forecasting  and

seasonal  climate  prediction ,  and  is  critical  to  disaster  mitigation .  Often  at  such  long  horizons ,  predicting

instantaneous  values  of  key  weather  variables  can  be  a  difficult  task  and  therefore  the  focus  is  often  on

averaged  value  of  key  weather  variables  over  a  certain  time  horizon ,  e . g .  weekly  average  precipitation .

Whereas  deep  learning  approaches  for  regional  or  S 2 S  tasks  are  scarce ,  most  of  the  recent  and  concurrent

work  focuses  on  global  forecasting  tasks .  Rasp  and  Thuerey   [RT2 1]  were  the  first  to  use  pretraining  on

climate  simulations  to  achieve  good  data-driven  medium-range  weather  prediction  with  a  ResNet  [He+ 1 6] ,

Weyn ,  Durran ,  et  al .  [WD C 20]  used  CNNs  on  a  cubed  sphere  for  global  weather  prediction ,  Weyn ,  Durran ,

et  al .   [Wey+ 2 1]  forecast  weather  sub-seasonally  with  a  large  ensemble  of  deep-learning  weather  prediction

models ,  Keisler   [Kei22]  applied  a  graph  neural  network  based  approach  to  weather  forecasting,  Ravuri ,  Lenc ,

et  al .   [Rav+ 2 1 ]  use  deep  generative  models  of  radar  for  precipitation  nowcasting,  Arcomano ,  Szunyogh ,  et  al .

[Arc+ 20]  build  a  reservoir  computing-based ,  low-resolution ,  global  prediction  model ,  and  MetNet  [Søn+ 20]

takes  as  input  radar  and  satellite  data  to  forecast  probabilistic  precipitation  maps .  These  approaches  are

complemented  by  general  machine  learning  models  for  fluid  dynamics  [Li+20 ;  Koc+2 1 ;  Lu+2 1 ;  Bra+22 ;

BWW22] .  Finally,  recent  state-of-the-art  neural  weather  models  such  as  FourCastNet  [Pat +22] ,  Pangu

weather   [Bi+ 22] ,  or  GraphCast   [Lam+ 22] ,  which  also  perform  global  forecasting  tasks ,  use  the  highest

°

resolution  0 . 25   ERA5  data,  and  are  optimized  on  the  respective  hardware  resources .

On  the  other  hand ,  climate  tasks  have  to  deal  with  much  longer  time  horizons .  Possible  categories  of  tasks

where  machine  learning  can  help  include  climate  proj ection  and  climate  model  downscaling:

•  C limate  proj ect ion  is  the  task  of  generating  estimates  of  climate  change  under  different  future  socio

economic  scenarios .  Usually,  this  takes  the  form  of  figuring  out  the  response  of  the  climate  system  to

different  forcing  factors  such  as  greenhouse  gases  and  aerosol  emissions .  Climate  proj ection  is  a  crucial  task

in  understanding  and  preparing  for  the  potential  impacts  of  climate  change .

While  the  application  of  machine  learning  in  this  field  is  still  in  its  early  st ages ,  recent  efforts  have  been

made  to  standardize  evaluation  in  this  domain .  One  example  of  this  is  ClimateBench   [WP + 22] ,  which  is  a

benchmark  dataset  drawing  on  CMIP6  to  provide  an  evaluation  framework  for  machine  learning  models

that  aim  to  improve  the  accuracy  of  climate  proj ections .  This  benchmark  aims  to  provide  a  consistent  and

reliable  evaluation  method  for  various  machine  learning  models  that  are  applied  to  climate  proj ections .

•  A  more  popular  application  of  ideas  in  machine  learning  is  towards  downscaling  of  climate  model .  Global

climate  models  typically  have  a  coarse  spatial  resolution ,  which  means  that  they  can  only  provide  a  rough

estimate  of  climate  conditions  at  a  local  or  regional  scale .  Moreover ,  the  simulations  often  reflect  systematic

biases  that  deviate  from  trends  in  the  observation  data.  The  aim  of  climate  model  downscaling  is  to

create  locally  accurate  climate  information  from  global  climate  proj ections  by  relating  those  to  observed

local  climatological  conditions .  This  process  improves  the  spatial  and  temporal  resolution  of  the  data,

making  it  more  suitable  for  use  in  local  and  regional  analyses .  Downscaling  methods  can  be  divided  into

7

ClimaX :  A  foundation  model  for  weather  and  climate

dynamic  approaches  that  relate  outputs  of  global  climate  models  with  those  of  regional  climate  models ,

and  statistical  approaches  that  infer  the  desired  transformations  using  data-driven  approaches  [WW97] .

Dynamic  approaches  are  physically  consistent ,  but  can  be  slow  and  have  large  biases ,  whereas  statistical

approaches  need  large  amounts  of  data  to  learn  expressive  mappings  that  hold  for  target  output  scenarios .

Similar  to  weather  forecasting ,  deep  learning  has  emerged  as  appealing  alternative  in  climate  science  as  well .

Recent  approaches  comprise  surrogate  models  to  emulate  climate  proj ections  [Web+20 ;  SM 1 9 ;  Sch 1 8 ;  B GS20 ;

Man+ 20] ,  extract  contextual  cues  from  existing  datasets  or  simulations  [Rei+ 1 9 ;  Hun+ 1 9 ;  Sch+ 1 7] ,  and

perform  climate  model  downscaling  [Sac+ 1 8 ;  Van+ 1 7;  BMMG20] .  Climate  model  downscaling  usually  inputs

low-resolution  reanalysis  data  and  local  orographic  information  to  obtain  high-resolution  local  information .

Many  recent  approaches  are  based  on  convolutional  architectures  [Höh+20 ;  Vau+2 1 ;  Mar+22] .

2 . 3 .  Fou ndation  models

Bommasani ,  Hudson,  et  al.  [Bom+2 1]  gave  the  term “  foundation  models”  to  the  emerging  paradigm  of

training  scalable  deep  learning  models  on  broad  data  via  self-supervision  which  could  then  be  adapted  (often

via  finetuning)  to  a  wide  range  of  downstream  tasks .  Current  notable  examples  include  BERT  [Dev+ 1 8] ,

GPT  [Bro+20]  and  PaLM  [Cho+22] ,  in  language ,  CLIP  [Rad+2 1] ,  Florence  [Yua+2 1] ,  BEiT  [Wan+22]  for

vision-language .  Outside  applications  on  data  crawled  from  web ,  this  paradigm  has  also  started  finding

success  in  various  scientific  domains  like  protein  design   [Ver+ 22] .  Key  significance  of  such  models  has  been

identified  as  emergence  with  respect  to  model  capabilities  and  homogenization  with  respect  to  methodologies

for  different  tasks ,  domains ,  and  modalities ,  enabled  by  the  principles  of  transfer  learning   [TP 1 2]  at  scale .

While  a  foundation  model  itself  should  be  considered  incomplete ,  it  can  provide  a  common  basis  from  which

various  task-specific  models  can  be  derived .  Current  research  at  the  intersection  of  weather  and  climate

science  and  ML  has  largely  focused  on  designing  separate  models  for  every  task  of  interest  despite  potential

availability  of  fairly  diverse  large  scale  data  with  shared  underlying  physics  and  geology  across  these  tasks .

A  few  recent  works  have  proposed  pretraining  techniques  for  satellite  imagery  and  remote  sensing  [YL20 ;

Con+ 22 ;  Ree+ 22a]  but  they  have  so  far  not  been  applied  to  multi-sensory  data  and  variables  in  weather  and

climate .

3 .   A p proa c h

Given  the  availability  of  large  scale  data  sources ,  together  with  shared  physics  and  geology  between  various

weather  and  climate  tasks ,  we  aim  to  build  a  generalizable  deep  learning  foundation  model .  The  model  needs

to  be  able  to  input  heterogeneous  datasets  of  different  variables ,  and  provide  spatio-temporal  coverage  based

on  physical  groundings .  We ,  therefore ,  first  take  a  closer  look  at  input  representations ,  and  next  design  a

model  to  cope  with  their  heterogeneity  -  local ,  global ,  and  across  variables .

3 . 1 .  I n p ut  representation

We  are  interested  in  gridded  prediction  tasks ,  in  which  the  model  takes  an  input  of  shape  ��  ×   ��  ×   ��  and

predicts  an  output  of  shape  ��
 ′  ×   ��′  ×   ��′
 .  ��  refers  to  the  number  of  input  variables ,  which  can  be  weather

conditions  such  as  geopotential  and  temperature ,  or  climate  forcing  factors  such  as  C O 2  and  S O2 .  ��  and

��  refer  to  the  spatial  resolution  of  the  input  data,  which  depends  on  how  densely  we  grid  the  globe .  This

general  representation  captures  a  broad  variety  of  downstream  tasks  in  Earth  systems  science .  Similarly,

��
 ′
 ,  ��′
 ,  ��′
 refer  to  the  variables  and  spatial  resolution  of  the  predicted  outputs .  We  mainly  work  with  two

° °

spatial  resolutions :  5 . 62 5   ( 32  ×   64  grid  points)  and  1 . 4062 5   ( 1 28  ×   2 56  grid  points) .  Semantically,  a  ��  ×   ��

map  can  represent  the  entire  globe  or  a  specific  region  such  as  North  America.

3 . 2 .  M o del  arch itectu re

We  aim  to  design  a  foundation  model  that  we  can  pretrain  on  heterogeneous  data  sources  and  then  finetune

to  solve  various  downstream  weather  and  climate  tasks .  From  Section  3 . 1 ,  one  could  think  of  the  tasks  as

8

ClimaX :  A  foundation  model  for  weather  and  climate

### Ta rg ets at l ea d ti m e Δ��

## Va ria b l e  
 Va ria b l e  
 C l i m a X 
 ⋮

To ke n i zati o n 
 Ag g re g ati o n

Va r .  I D

t8 5 0

⊕

Va r . I D

⊕

###### u 5 00 
 Tra n sfo r m e r

# ⋮ 
 ℒ ( �� , ℬ )

⋮

Va r . I D

### ⊕

⊕

S ca l a r E m b e d

q 8 5 0 
 Patc h E m b e d

### Δ ��

Le a d ti m e

Po s i t i o n

C ross- atte nti o n

Figu re  2 :  Pretraining  phase  of  ClimaX .  Variables  are  encoded  using  variable-separate  tokenization ,  and

subsequently  aggregated  using  variable  aggregation.  Together  with  position  embedding  and  lead  time

embedding  those  are  fed  to  the  ViT  backbone .

image-to-image  translation  problems  with  ��  input  channels  and  ��
 ′  output  channels .  This  makes  any  image

architecture  a  natural  fit ,  such  as  UNet   [RFB 1 5] ,  ResNet   [He+ 1 6] ,  or  Vision  Transformers  (ViT)   [Dos+ 20] .

However ,  the  settings  of  climate  and  weather  tasks  are  much  broader ,  where  we  may  want  to  make  predictions

for  regional  or  even  spatially  incomplete  data,  forecast  unseen  climate  variables ,  or  finetune  the  model  on  data

at  different  resolutions  from  pretraining.  Current  CNN-based  architectures  are  not  applicable  in  these  scenarios ,

as  they  require  the  input  to  be  perfectly  gridded ,  contain  a  fixed  set  of  variables ,  and  have  a  fixed  spatial

resolution .  Transformers-based  architectures ,  on  the  other  hand ,  provide  much  better  flexibility  by  treating

the  image-like  data  as  a  set  of  tokens .  Therefore ,  we  build  ClimaX  architecture  upon  Vision  Transformers

(ViT)  [Dos+ 20 ;  Vas+ 1 7] ,  and  propose  two  maj or  architectural  changes ,  namely  variable  tokenization  and

variable   aggregation  to  further  improve  the  flexibility  and  generality,  which  we  will  describe  next .

3 . 2 . 1 .  Varia b le  to ken ization

Given  an  input  of  shape  ��  ×   ��  ×   �� ,  ViT  tokenizes  the  input  into  a  sequence  of  (��/��)  ×   ( ��/��)  =  ℎ  ×   ��

patches ,  with  each  patch  having  a  size  of  ��  ×   ��
2
 ,  where  ��  is  the  patch  size .  This  tokenization  scheme  works

well  for  image  data,  as  ��  is  always  the  RGB  channels ,  which  is  the  same  for  all  datasets .  However ,  this  is  not

true  for  climate  and  weather  data,  where  the  number  of  physical  variables  can  vary  between  different  datasets .

For  example ,  in  the  CMIP6  proj ect   [Eyr+ 1 6] ,  each  dataset  contains  simulated  data  of  a  different  climate

model ,  and  thus  has  a  different  set  of  underlying  variables .  Therefore ,  we  propose  variable  tokenization,  a

novel  tokenization  scheme  that  tokenizes  each  variable  in  the  input  separately.  Specifically,  each  input  variable

as  a  spatial  map  of  shape  ��  ×   ��  is  tokenized  into  a  sequence  of  ℎ  ×   ��  patches ,  which  results  in  ��  ×   ℎ  ×   ��

patches  in  tot al .  Finally,  each  input  patch  of  size  ��
2
 is  linearly  embedded  to  a  vector  of  dimension  �� ,  where

��  is  the  chosen  embedding  size .  The  output  of  the  variable  tokenization  module  therefore  has  a  dimension  of

��  ×   ℎ  ×   ��  ×   �� .  Figure  3  illustrates  our  proposed  tokenization  scheme .

3 . 2 . 2 .  Varia b le  aggregation

While  variable  tokenization  allows  ClimaX  to  learn  from  datasets  with  varying  numbers  of  input  variables ,

it  has  two  inherent  problems .  First ,  it  results  in  a  sequence  of  length  ��  ×   ℎ  ×   ��  which  increases  linearly

with  the  number  of  variables .  Since  we  use  attention  to  model  the  sequence ,  the  memory  complexity  scales

quadratically  with  the  number  of  variables .  This  is  computationally  expensive ,  as  we  can  have  up  to  48  input

9

ClimaX :  A  foundation  model  for  weather  and  climate

�� 
 ⋮ 
 ⋮ 
 ⋮ 
 ⋮

��

# T2 m   U 5 0 0   Q8 5 0

F igu re  3 :  Variable  tokenization .  Each  variable  is  independently  tokenized .

variables  in  our  experiments .  Moreover ,  because  we  tokenize  each  variable  separately,  the  input  sequence  will

contain  tokens  of  different  variables  with  very  different  physical  groundings ,  which  can  create  difficulties  for

the  attention  layers  to  learn  from .  We  therefore  propose  variable   aggregation  to  solve  the  two  mentioned

challenges .  For  each  spatial  position  in  the  ℎ  ×   ��  map ,  we  perform  a  cross-attention  operation ,  in  which

the  query  is  a  learnable  vector ,  and  the  keys  and  values  are  the  ��  embedding  vectors  of  ��  variables  at  that

position .  The  cross-attention  module  outputs  a  single  vector  for  each  spatial  position ,  thus  reducing  the

sequence  length  to  ℎ  ×   �� ,  significantly  lowering  the  computational  cost .  Moreover ,  the  sequence  now  contains

unified  tokens  with  universal  semantics ,  creating  an  easier  task  for  the  attention  layers .  Figure  4  shows  our

proposed  variable  aggregation .

�� 
 ⋮
 ⋮
 ⋮
 ⋮

��

# T2 m   U 5 0 0   Q8 5 0

F igu re  4 :  Position-based  variable  aggregation  reduces  a  sequence  of  length  ��  ×   ℎ  ×   ��  to  ℎ  ×   �� .

3 . 2 . 3 .  Tra nsformer

Post  variable  aggregation ,  we  need  a  sequence  model  for  generating  the  output  tokens .  While  in  principle ,  one

could  use  any  general  sequence  model ,  we  propose  to  extend  a  standard  Vision  Transformer  (ViT) .  Moreover ,

since  the  standard  ViT  treats  image  modeling  as  pure  sequence-to-sequence  problems ,  it  can  perform  tasks

that  some  other  variations  cannot   [Liu+ 2 1 ;  Liu+ 22] ,  such  as  learning  from  spatially  incomplete  data,  where

the  input  does  not  necessarily  form  a  complete  grid .  This  is  useful  in  the  regional  forecasting  task  we  consider

in  Section  4 . 2 . 2 .  In  the  experiments ,  we  report  results  with  8  attention  layers ,  an  embedding  size  of  1 024 ,  and

a  hidden  dimension  of  1 024  ×   4 .  After  the  attention  layers ,  we  employ  a  prediction  head  that  takes  a  token

and  outputs  a  vector  of  size  ��
 ′  ×   ��
2
 .  The  prediction  head  is  a  2-layer  MLP  with  a  hidden  dimension  of  1 024 .

We  provide  more  details  in  Appendix  A .

3 . 3 .  D atasets

3 . 3 . 1 .   P ret ra i n i ng

We  believe  that  CMIP6 ’ s  diversity  and  scale  presents  an  attractive  opportunity  for  pretraining  large-scale

foundation  models .  However ,  handling  the  inconsistent  set  of  variables  across  different  data  sources  can  be  a

challenge .  In  this  work  we  only  use  a  subset  of  variables  from  five  different  data  sources  (MPI-ESM ,  TaiESM ,

AWI-ESM ,  HAMMOZ ,  CMC C )  containing  global  proj ections  of  climate  scenarios  from  1 850  to  20 1 5  with  the

time  delt a  of  6  hours  as  described  in  Table  8 .  Due  to  variable  original  resolution ,  we  choose  to  simplify  our

data-loading  by  regridding  them  to  commonly  used  resolutions   [Ras+ 20 ;  RT2 1]  of  5 . 625 °  (32  ×   64  grid  points)

and  1 . 406 2 5 °  ( 1 2 8  ×   2 5 6  grid  p oint s ) 1
 .

1 Regridding  was  done  using  the  xesmf  Python  package  [Zhu 1 8]  using  bilinear  interpolation .

1 0

ClimaX :  A  foundation  model  for  weather  and  climate

3 . 3 . 2 .  F i netu n i ng  a n d  eva l uation

We  use  the  ERA5  reanalysis  data  as  described  in  Appendix  C . 2 ,  as  the  source  of  datasets  for  finetuning  and

evaluation  for  various  weather  related  downstream  tasks .  Due  to  its  large  size ,  it  is  common  to  regrid   [Ras+ 20 ;

RT2 1 ]  the  high-resolution  dat a  to  lower  resolutions  like  5 . 62 5 °  ( 32  ×   64  grid  points)  and  1 . 4062 5 °  ( 1 28  ×   2 56

grid  points)  to  fit  within  the  available  computational  constraints 2
 .  We  follow  the  evaluation  procedure  by  Rasp

and  Thuerey   [RT2 1]  and  use  this  data  to  assess  the  forecasting  performance  of  our  ML  models  at  different

lead  time  horizons .  More  details  about  the  individual  datasets  are  in  their  appropriate  experiment  sections .

3 . 4 .   Tra i n i n g

3 . 4 . 1 .   P ret ra i n i ng

We  pretrain  ClimaX  on  CMIP6  data  to  predict  future  weather  conditions  given  the  current  conditions .  That

is ,  given  the  weather  snapshot  ����  of  shape  ��  ×   ��  ×   ��  at  a  particular  time  �� ,  ClimaX  learns  to  predict  the

future  weather  scenario  ����+Δ��  of  the  same  shape  at  lead  time  ∆�� .  To  obtain  a  pretrained  model  that  is

generally  applicable  to  various  temporal  forecasting  tasks ,  we  randomize  the  lead  time  from  6  hours  to  1 68

hours   (i . e . ,  1  week)  during  pretraining .  We  add  the  lead  time  embedding  to  the  tokens  to  inform  the  model  of

how  long  it  is  forecasting  into  the  future .  The  lead  time  embedding  module  is  a  single-layer  MLP  that  maps  a

scalar  to  a  vector  of  the  embedding  size  �� .  Figure  2  depicts  the  forward  pass  of  ClimaX  in  pretraining .  For

an  input  ���� ,  we  sample  a  lead  time  ∆��  ∼ ��  [6 ,  1 68]   and  get  the  corresponding  ground  truth  ����+Δ�� .  Input

variables  are  tokenized  separately  using  variable  tokenization ,  and  are  subsequently  aggregated  at  each  spatial

location ,  resulting  in  a  sequence  of  ℎ  ×   ��  unified  tokens .  We  add  the  tokens  with  the  lead  time  embedding

and  positional  embedding  before  feeding  the  sequence  to  the  ViT  backbone .  The  output  of  the  last  attention

layer  is  fed  to  a  prediction  head ,  which  transforms  the  sequence  back  to  the  original  shape  of  ��  ×   ��  ×   �� .

W˜ e  employ  the  latitude-weighted  mean  squared  error  [Ras+ 20]  as  our  obj ective  function .  Given  the  prediction

��
��+Δ��  and  the  ground  truth  ����+Δ�� ,  the  loss  is  computed  as :

��
 
 ��
 
 ��
 
 ˜

= 1 
   �� , �� , ��
 − �� , �� , ��
 2

ℒ  
 ∑︁ ∑︁ ∑︁ �� (��) (����+Δ��    ��
��+Δ��
)
 ,  ( 1 )

��  ×   ��  ×   ��
 = = =

�� 1 
 �� 1 
 �� 1

in  which  �� (�� )   is  the  latitude  weighting  factor :

cos ( lat ( �� ) )

�� ( �� )  =  1
 ��
 ′ ,   ( 2 )

��
 ∑︀ ��
′ = 1  cos ( lat ( ��
 ) )

where  lat (�� )   is  the  latitude  of  the  corresponding  ��th  row  of  the  grid .  The  latitude  weighting  term  accounts  for

the  non-uniformity  in  areas  when  we  grid  the  round  globe .  Grid  cells  toward  the  equator  have  larger  areas

than  the  cells  near  the  pole ,  and  thus  should  be  assigned  more  weights .

3 . 4 . 2 .  F i n et u n i ng

ClimaX  has  four  learnable  components ,  including  the  token  embedding  layers ,  the  variable  aggregation  module ,

the  attention  blocks ,  and  the  prediction  head .  We  evaluate  the  performance  of  ClimaX  on  various  downstream

tasks ,  which  we  categorize  into  two  finetuning  scenarios :  one  in  which  the  downstream  variables  belong  to

the  set  of  pretraining  variables ,  and  the  other  with  variables  unseen  during  pretraining .  In  the  first  case ,  we

finetune  the  entire  model ,  and  in  the  latter ,  we  replace  the  embedding  layers  and  the  prediction  head  with

newly  initialized  networks ,  and  either  finetune  or  freeze  the  other  two  components .  We  present  more  details

of  each  downstream  task  in  Section  4 .

2 Regridding  was  done  using  the  xesmf  Python  package  [Zhu 1 8]  using  bilinear  interpolation .

1 1

ClimaX :  A  foundation  model  for  weather  and  climate

4 .   Ex p e ri m e nts

We  finetune  ClimaX  on  a  diverse  set  of  downstream  tasks  to  evaluate  its  performance  and  generality.  We

categorize  the  tasks  into  forecasting,  climate  proj ection ,  and  climate  downscaling.  The  experiments  aim  to

answer  the  following  questions :

•  How  does  ClimaX  perform  on  global  forecasting  compared  to  the  current  state-of-the-art  NWP  system?

•  Can  we  finetune  ClimaX  to  make  forecasts  for  a  specific  region  or  at  different  temporal  horizons  from

pretraining?

•  How  well  does  ClimaX  perform  on  climate  tasks  that  are  completely  different  from  pretraining?

In  addition  to  the  main  experiments ,  we  analyze  the  scaling  property  of  ClimaX ,  i . e . ,  how  the  performance

of  ClimaX  improves  with  increasing  data  size ,  model  capacity,  and  data  resolution .  Finally,  we  perform

comprehensive  ablation  studies  to  understand  the  trade-off  between  computation  and  performance  when

finetuning  ClimaX.

4 . 1 .  N e u ra l  ba se l i n es

In  global  forecasting,  we  compare  ClimaX  with  IFS   [Wed+ 1 5] ,  the  current  gold  standard  in  weather  forecasting.

In  tasks  we  do  not  have  a  baseline ,  we  compare  with  UNet  [RFB 1 5 ;  GB 22]  and  ResNet  [He+ 1 6] ,  two  CNN

baselines  commonly  used  in  vision  tasks .  We  borrow  the  ResNet  architecture  from  Weatherbench  [Ras+20] .

The  exact  architectural  details  of  these  baselines  are  in  Appendix  A . 2 .

4 . 2 .  Forecasti ng

4 . 2 . 1 .  G loba l  forecasti ng

Given  global  weather  conditions  ����  at  a  particular  time  �� ,  we  want  to  forecast  the  weather  at  a  future  time

����+Δ�� ,  in  which  ∆��  is  the  lead  time .  The  input  variables  include  6  atmospheric  variables  at  7  vertical  levels ,

3  surface  variables ,  and  3  const ant  fields ,  resulting  in  48  input  variables  in  tot al .  The  det ails  of  the  variables

are  in  Table  9 .  We  evaluate  ClimaX  on  predicting  four  target  variables :  geopotential  at  500hPa  ( Z 500) ,  the

temperature  at  850hPa  (T850) ,  the  temperature  at  2  meters  from  the  ground  (T2m) ,  and  zonal  wind  speed

at  1 0  meters  from  the  ground  (U 1 0) .  Z500  and  T850  are  the  two  standard  verification  variables  for  most

medium-range  NWP  models  and  are  often  used  for  benchmarking  in  previous  deep  learning  works ,  while  the

two  surface  variables ,  T2m  and  U 1 0 ,  are  relevant  to  human  activities .  We  consider  seven  lead  times :  6  hours ,

{ 1 ,  3 ,  5 ,  7}  days ,  2  weeks ,  and  1  month ,  which  range  from  nowcasting  to  short  and  medium-range  forecasting

and  beyond .  We  consider  predicting  each  target  variable  at  each  lead  time  a  separate  task ,  and  finetune  a

separate  model  for  each  task .  We  discuss  alternative  finetuning  protocols  in  Section  4 . 6 .

° °

We  compare  ClimaX  with  IFS  and  the  two  CNN  baselines  on  the  ERA5  dataset  at  both  5 . 625   and  1 . 40625

resolutions .  Following   [Ras+ 20] ,  we  split  the  dat a  into  three  sets ,  in  which  the  training  dat a  is  from  1 9 79  to

20 1 5 ,  the  validation  dat a  is  in  20 1 6 ,  and  the  test  dat a  is  in  20 1 7  and  20 1 8 .  We  finetune  ClimaX  and  train

the  other  deep  learning  baselines  using  the  latitude-weighted  MSE  loss  in  Equation  ( 1 ) .  We  perform  early

stopping  on  the  validation  loss  for  all  deep  learning  models ,  and  evaluate  the  best  checkpoint  on  the  test  set .

For  IFS ,  we  download  the  predictions  from  the  TIG GE  archive   [Bou+ 1 0]  for  the  year  20 1 83
 .  We  compare  all

methods  on  latitude-weighted  root  mean  squared  error  (RMSE)  and  latitude-weighted  anomaly  correlation

coefficient  (AC C ) ,  two  commonly  used  metrics  in  previous  works .  The  formulations  of  the  two  metrics  are  in

Appendix  D . 1 .  Lower  RMSE  and  higher  AC C  indicates  better  performance .

° °

Figures  5  and  6  show  the  performance  of  ClimaX  and  the  baselines  at  5 . 62 5   and  1 . 4062 5 ,  respectively.  At  low

resolution ,  IFS  outperforms  ClimaX  on  6-hour  to  5-day  prediction  tasks .  On  longer  horizons ,  however ,  ClimaX

performs  comparably  to  or  slightly  better  than  IFS ,  especially  on  1 4-day  prediction .  At  higher  resolution ,  the

3We  were  not  able  to  download  IFS  predictions  for  20 1 7  due  to  some  server  issues .

1 2

ClimaX :  A  foundation  model  for  weather  and  climate

Z5 0 0  [ m 2
/s2
]
 T2 m  [ K]
 T85 0  [ K]
 U 1 0  [ m/s]

3 . 5

1 0 0 0
 3 . 0
 4
 5

8 0 0
 2 . 5
 4

3

SE 
 60 0
 2 . 0
 3

MR ttre 
 1 . 5
 2

40 0
 be  2

is  1 . 0

2 0 0
 re 1 
 1

low 0 . 5

0 
 0 . 0 
 0 
 0

1 . 0 
 1 . 0 
 1 . 0 
 1 . 0

re

tte

bis   0 . 8

0 . 8
 r 0 . 8
 0 . 8

C 
 hieg

C h 0 . 6

A 0 . 6
 0 . 6
 0 . 6

0 . 4

0 . 4
 0 . 4
 0 . 4

0 . 2

1   3  5  7  1 0  1 4  3 0
 1   3  5  7  1 0  1 4  3 0
 1   3  5  7  1 0  1 4  3 0
 1   3  5  7  1 0  1 4  3 0

Leadti m e  [d ays]

C l i m aX  (5 . 625 °)  I FS  (5 . 625 °)  Res N et  (5 . 625 °)  U N et  (5 . 625 °)

°

F igu re  5 :  Performance  on  global  forecasting  on  ERA5  at  5 . 62 5 .

performance  of  ClimaX  closely  matches  that  of  IFS  even  for  short  horizons ,  and  is  superior  in  forecasting

at  7  days  and  beyond .  The  trends  are  similar  for  both  RMSE  and  AC C .  The  two  CNN  baselines  perform

similarly  and  achieve  reasonable  performance ,  but  lag  behind  ClimaX  and  IFS  on  all  tasks .  We  include  other

additional  task-specific  baselines   [Pat + 22 ;  Bi+ 22 ;  Lam+ 22]  in  Appendix  D . 2 .  These  baselines  are  trained  on

higher-resolution  ERA5  ( 0 . 2 5 ° )  so  are  not  directly  comparable .

4 . 2 . 2 .  Regiona l  forecasti ng

It  is  not  always  possible  to  make  global  predictions ,  especially  when  we  only  have  access  to  regional  data  In

this  section ,  we  evaluate  ClimaX  on  regional f  orecasting  of  the  relevant  variables  in  North  America,  where  the

task  is  to  forecast  the  future  weather  in  North  America  given  the  current  weather  condition  in  the  same  region .

°

We  create  a  new  dataset  from  the  ERA5  data  at  1 . 4062 5   that  has  the  same  set  of  variables  but j  ust  focuses

on  the  North  America  region .  We  call  this  dataset  ERA5-NA  and  present  details  of  how  to  construct  it  in

Appendix  C . 2 .  Training ,  validation ,  and  test  splits  are  done  similarly  to  Section  4 . 2 . 1 .  Figure  7  illustrates  the

finetuning  process  of  ClimaX  on  this  task,  where  the  only  difference  from  global  forecasting  is  the  input  now

only  contains  tokens  that  belong  to  North  America.

Since  the  task  has  not  been  considered  in  previous  works ,  we  compare  ClimaX  with  the  two  CNN  baselines

ResNet  and  UNet ,  and  the  scratch-trained  version  of  ClimaX ,  which  we  refer  to  as  Cli-ViT .  In  addition ,

°

we  finetune  two  ClimaX  models ,  in  which  one  was  pretrained  on  CMIP6  at  1 . 40625 ,  and  the  other  was

pretrained  on  5 . 625 °  data.  To  finetune  the  low-resolution  model  on  higher-resolution  data,  we  follow  the

common  practice  of  interpolating  the  positional  embedding  [Dos+20 ;  Tou+2 1] .  We  denote  this  model  as

ClimaX-pos-interp .  We  evaluate  all  methods  on  predicting  Z 500 ,  T2m ,  and  T850  at  lead  times  of  3 ,  5 ,  and  7

days .  Latitude-weighted  RMSE  is  used  as  the  evaluation  metric .

Figure  8  compares  the  performance  of  ClimaX  and  the  baselines .  ClimaX  is  the  best  performing  method

among  different  target  variables  and  lead  times .  Interestingly,  even  though  pretrained  on  data  at  a  lower

resolution ,  ClimaX-pos-interp  achieves  the  second  best  performance  in  predicting  Z500  and  T850 ,  and  only

underperforms  ResNet  in  predicting  T2m  at  3-day  lead  time .  This  result  shows  that  ClimaX  can  gain  strong

1 3

ClimaX :  A  foundation  model  for  weather  and  climate

Z5 0 0  [ m 2
/s2
]
 T2 m  [ K]
 T85 0  [ K]
 U 1 0  [ m/s]

3 . 5

1 0 0 0
 3 . 0
 4
 5

8 0 0
 2 . 5
 4

3

SE 
 60 0
 2 . 0
 3

MR ttre 
 1 . 5
 2

40 0
 be  2

is  1 . 0

2 0 0
 re 1 
 1

low 0 . 5

0 
 0 . 0 
 0 
 0

1 . 0 
 1 . 0 
 1 . 0 
 1 . 0

re

tte

bis   0 . 8

0 . 8
 r 0 . 8
 0 . 8

C 
 hieg

C h 0 . 6

A 0 . 6
 0 . 6
 0 . 6

0 . 4

0 . 4
 0 . 4
 0 . 4

0 . 2

1   3  5  7  1 0  1 4  3 0
 1   3  5  7  1 0  1 4  3 0
 1   3  5  7  1 0  1 4  3 0
 1   3  5  7  1 0  1 4  3 0

Leadti m e  [d ays]

C l i m aX  ( 1 . 40625 °)  I FS  ( 1 . 40625 °)

°

F igu re  6 :  Performance  on  global  forecasting  on  ERA5  at  1 . 40625 .

performance  on  tasks  that  have  different  spatial  coverage  or  even  different  spatial  resolution  from  pretraining.

4 . 2 . 3 .  S u b-seasona l  to  seasona l  cu m u lative  pred iction

Sub-seasonal  to  seasonal   ( S 2 S )  prediction  is  the  task  of  forecasting  at  a  time  range  between  2  weeks  and

2  months  [VR1 8] ,  which  bridges  the  gap  between  weather  forecasting  and  climate  proj ection .  Compared

to  the  other  two  well-established  tasks ,  S 2 S  prediction  has  received  much  less  attention ,  despite  having  a

significant  socioeconomic  value  in  disaster  mitigation.  Recent  works  have  proposed  data-driven  approaches

based  on  both  traditional  machine  learning  [Hwa+ 1 9 ;  Pro+ 1 8 ;  TL 1 8]  and  deep  learning  [Wey+2 1 ;  Zho+2 1 ;

Ore+ 1 9] ,  but  their  performances  often  lag  behind  adaptive  bias  correction  methods  [Mou+23a]  on  standard

benchmarks   [Mou+ 23b] .  Here ,  following  the  S 2 S  competition  (https : / /s2s-ai-challenge . github . io/ ) ,  we  aim  to

predict  the  biweekly  average  statistics  of  weeks  3-4  and  weeks  5-6 ,  which  correspond  to  lead  times  of  2  weeks

and  4  weeks ,  respectively.  We  construct  ERA5- S 2 S ,  a  new  dataset  from  5 . 625 °  ERA5  that  has  the  same  input

variables ,  but  the  output  variables  are  averaged  from  the  lead  time  to  2  weeks  ahead  into  the  future .

We  compare  ClimaX  with  ResNet ,  UNet ,  and  Cli-ViT  on  the  S 2 S  prediction  of  four  target  variables :  T850 ,

T2m ,  U 1 0 ,  and  V 1 0 .  Table  1  compares  the  RMSE  of  ClimaX  and  the  baselines .  ClimaX  achieves  the  lowest

error  for  all  variables ,  and  the  performance  gap  with  the  best  baseline  UNet  is  larger  at  increasing  lead  times .

ClimaX  also  has  significant  performance  gains  over  its  scratch-trained  counterpart  Cli-ViT ,  showing  the

effectiveness  of  our  pretraining  procedure  in  capturing  features  that  are  generally  useful  for  various  temporal

prediction  t asks .

4 . 3 .  C l i mate  projection

To  further  test  the  generality  of  ClimaX ,  we  evaluate  the  model  on  ClimateBench  [WP +22] ,  a  recent  benchmark

designed  for  testing  machine  learning  models  for  climate  proj ections .  The  goal  of  ClimateBench  is  to  predict

the  annual  mean  global  distributions  of  surface  temperature ,  diurnal  temperature  range ,  precipitation ,  and

the  90th  percentile  of  precipitation ,  given  the  four  anthropogenic  forcing  factors :  carbon  dioxide   ( C O 2 ) ,  sulfur

dioxide  ( S O 2 ) ,  black  carbon  (B C ) ,  and  methane  ( CH4 ) .  We  note  that  this  is  not  a  temporal  modeling  task ,

14

ClimaX :  A  foundation  model  for  weather  and  climate

### Ta rg ets at l ea d ti m e Δ��

T8 5 0

## U500 
 C l i m a X 
 ℒ �� ℬ

# ( , )

⋮

Q8 5 0

### Δ ��

Le a d ti m e

Po s i t i o n

S ca l a r E m b e d

F igu re  7 :  Finetuning  setup  for  Regional  Forecasting  in  North  America.

Z5 0 0  [ m 2
/s2
]
 T2 m  [ K]
 T85 0  [ K]

2 . 0

E 1 0 0 0
 1 . 8
 3 . 5

SM 1 . 6

R 800
 3 . 0

1 . 4

1 . 2
 2 . 5

3  5  7
 3  5  7
 3  5  7

Leadti m e  [d ays]

C l i m aX- pos- i nte rp  (5 . 625 °)  C l i m aX  ( 1 . 40625 °)  C l i -ViT  ( 1 . 40625 °)  Res N et  U N et

Figu re  8 :  Performance  on  Regional  (North  America)  forecasting  for  key  variables .

as  we  do  not  predict  the  future  given  the  past .  Instead ,  we  answer  questions  like  what  will   be  the   annual

mean  temperature f  or   a  specified   CO2  level ?  In  particular ,  note  that  the  input  variables  and  the  task  itself  are

completely  different  from  pretraining.

Figure  9  illustrates  the  finetuning  pipeline  of  ClimaX  for  ClimateBench .  As  the  input  and  output  variables

are  unseen  during  pretraining,  we  replace  the  pretrained  embedding  layers  and  prediction  heads  with  newly

initialized  networks ,  while  keeping  the  attention  layers  and  the  variable  aggregation  module .  We  consider  two

finetuning  protocols ,  in  which  we  either  freeze4
 ( ClimaXfrozen )  or  finetune  ( ClimaX)  the  attention  layers .  In

addition ,  we  introduce  two  components  to  the  pipeline  in  Figure  2 .  We  use  a  history  of  the  preceding  ten  years

of  the  forcing  factors  to  make  predictions  for  a  particular  year ,  creating  an  input  of  shape  ��  ×   ��  ×   ��  ×   �� .

Each  time  slice  of  the  input  goes  through  variable  tokenization ,  variable  aggregation ,  and  the  attention  layers

as  usual ,  which  output  a  feature  tensor  of  shape  ��  ×   ℎ  ×   ��  ×   �� ,  where  ��  is  the  embedding  size .  The  feature

tensor  then  goes  through  a  global  average  pooling  layer ,  reducing  the  dimension  to  ��  ×   �� .  Finally,  the  1 0-year

history  is  aggregated  using  a  cross-attention  layer  before  being  fed  to  the  prediction  head ,  which  linearly

transforms  the  ��-dimensional  feature  vector  to  a  ��  × ��  map .  The  history  aggregation  and  the  global  pooling

modules  are  the  two  additions  to  the  original  ClimaX  architecture .  These  architectural  designs  are  inspired

by  the  neural  network  baseline  in   [WP + 22] .

We  compare  ClimaX  with  ClimaXfrozen ,  Cli-ViT ,  and  the  best  baseline  from  ClimateBench.  Following  [WP +22] ,

we  use  the  standard  mean  squared  error  (Equation  ( 1 )  without  the  weighting  term)  as  the  loss  function .

We  evaluate  all  methods  on  RMSE ,  NRMSE��  (Spatial) ,  NRMSE��  (Global) ,  and  Total  =  NRMSE��  +  5  ×

NRMSE��  [WP + 22] .  Details  of  the  metrics  are  in  Appendix  D . 1 .  Table  2  shows  the  results .  ClimaXfrozen

performs  the  best  in  predicting  two  temperature-related  variables ,  followed  by  ClimaX .  This  shows  that

4We  finetune  the  LayerNorm  in  ClimaXfrozen ,  as  suggested  by  Lu ,  Grover ,  et  al .   [Lu+ 22] .

1 5

ClimaX :  A  foundation  model  for  weather  and  climate

Ta ble  1 :  RMSE  of  ClimaX  and  baselines  on  5 . 625 °  ERA5- S 2 S  prediction  tasks .

T850  T2m  U 1 0  V1 0

Weeks  3-4  Weeks  5-6  Weeks  3-4  Weeks  5-6  Weeks  3-4  Weeks  5-6  Weeks  3-4  Weeks  5-6

Resnet  2 . 1 2  2 . 1 3   1 . 8 8  2 . 1 6   1 . 9 1   1 . 94   1 . 5 2   1 . 5 9

Unet   1 . 9 1   1 . 9 5   1 . 6 7   1 . 79   1 . 8 5   1 . 9 0   1 . 5 2   1 . 5 7

C li-ViT   1 . 9 6   1 . 9 6   1 . 79   1 . 9 0   1 . 8 3   1 . 9 2   1 . 5 1   1 . 5 6

ClimaX   1 . 8 9  1 . 9 2  1 . 6 6  1 . 70  1 . 8 1  1 . 8 6  1 . 5 0  1 . 54

S O 2

#### Va ria b l e  
 Va ria b l e  
 C l i m a X

To ke n i zati o n 
 Ag g re g ati o n

Va r . I D 
 H i sto ry

Ag g re g ati o n

###### Ave ra g e s u rfa ce te m pe ratu re

⊕

⋮ 
 ⋮

# ⊕

Va r .   I D

lign d
a

### Tra nsfo rm e r 
 oo e ℒ �� , ℬ

## P H ( )

⊕ 
 S ca l a r E m b e d

B C 
 Patc h E m b e d

##### Fo rci n g  Fa cto rs 
 Cross-atte nti o n

Po s it i o n   Ti m e  h i st .

F igu re  9 :  Finetuning  pipeline  for  ClimateBench .  A  different  set  of  input  and  output  variables  requires

different  embedding  layers  and  prediction  heads .  Attention  layers  can  be  frozen  or  finetuned .

the  pretrained  attention  layers  can  serve  as  a  strong  feature  extractor  in  seemingly  unrelated  tasks .  Where

downstream  data  is  scarce  ( ClimateBench  has  only  754  data  points) ,  further  finetuning  the  attention  layer  can

lead  to  overfitting  and  thus  slightly  hurt  the  performance .  In  two  precipitation-related  tasks ,  ClimaX frozen

slightly  underperforms  ClimateBench  baseline  in  terms  of  NRMSE��  and  NRMSE��  but  outperforms  on  RMSE .

We  hypothesize  that  this  was  because  ClimaX  did  not  observe  the  precipitation  variable  during  pretraining,

which  has  very  different  behaviors  from  other  variables .

4 . 4 .  C l i mate  model  downsca l i ng

Climate  models  are  often  run  at  coarse  grids  due  to  their  high  computational  cost .  Although  these  predictions

are  useful  in  understanding  large-scale  climate  trends ,  they  do  not  provide  sufficient  detail  to  analyze  regional

and  local  phenomena.  Downscaling  aims  to  obtain  higher-resolution  proj ections  and  reduce  biases  from  the

outputs  of  these  models .  To  evaluate  the  applicability  of  ClimaX  to  the  task  of  climate  model  downscaling,

we  construct  a  new  dataset  based  on  CMIP6  and  ERA5  data  sources  for  coarse  inputs  and  higher  resolution

targets .  Specifically,  we  use  all  MPI-ESM ,  a  dataset  from  CMIP 6 ,  and  its  variables  listed  in  Table  8  at  5 . 62 5 °

°

as  input ,  and  train  separate  models  to  downscale  to  each  ERA5  target  variable  at  1 . 40625 .  We  compare

ClimaX  with  Cli-ViT  and  the  two  CNN  baselines ,  UNet  and  ResNet ,  as  most  recent  deep  downscaling

methods  [Van+ 1 7;  Rod+ 1 8 ;  Höh+20 ;  VKG 1 9 ;  LGD20]  are  based  on  convolution.  We  were  not  able  to

compare  with  YNet  [LGD 20] ,  the  current  best  method  on  deep  downscaling  as  we  did  not  have  access  to

high-resolution  auxiliary  data  such  as  elevation  and  topographical  information .  For  all  methods ,  we  first

bilinearly  interpolate  the  input  to  match  the  resolution  of  the  desired  output  before  feeding  it  to  the  model .

We  evaluate  all  methods  on  RMSE ,  Pearson  correlation,  and  Mean  bias ,  which  were  commonly  used  in  existing

deep  downscaling  works   [Van+ 1 7;  LGD 20] .  Details  of  the  metrics  are  in  Appendix  D . 1 .

Table  3  compares  ClimaX  and  the  baselines  quantitatively.  ClimaX  achieves  the  lowest  RMSE  and  a  mean

1 6

ClimaX :  A  foundation  model  for  weather  and  climate

Ta ble  2 :  Performance  of  ClimaX  and  the  baselines  on  ClimateBench .  Spatial  and  Global  denote  the  normalized

root  mean  squared  error  NRMSE��  and  the  NRMSE  of  the  global  mean  NRMSE�� ,  respectively.  Total  is  a

weighted  combination  of  Spatial  and  Global .

Surface  temperature  Diurnal  temperature  range  Precipitation  90th  percentile  precipitation

Spatial  Global  Total  RMSE  Spatial  Global  Total  RMSE  Spatial  Global  Total  RMSE  Spatial  Global  Total  RMSE

ClimateBench-NN   (reproduced)  0 . 1 23  0 . 080  0 . 5 24  0 . 404  7 . 465  1 . 233  1 3 . 632  0 . 1 50  2 . 349   0 . 1 5 1  3 . 1 04   0 . 5 53  3 . 1 08   0 . 2 8 2   4 . 5 1 7  1 . 594

ClimateBench-NN  (paper)  0 . 1 07  0 . 044  0 . 32 7  N/A  9 . 9 1 7  1 . 372  1 6 . 778  N/A   2 . 1 2 8   0 . 209  3 . 1 75  N/A   2 . 6 1 0   0 . 346   4 . 339   N/A

C li-ViT  0 . 086  0 . 044  0 . 30 5  0 . 36 2  6 . 99 7   1 . 75 9   1 5 . 79 2  0 . 1 46  2 . 2 24  0 . 24 1  3 . 430  0 . 5 5 0  2 . 800  0 . 3 2 9  4 . 447   1 . 5 79

ClimaX  0 . 086   0 . 043   0 . 300  0 . 36 2  7 . 1 48  0 . 96 1  1 1 . 9 5 2  0 . 1 47  2 . 360  0 . 206  3 . 390  0 . 5 54  2 . 739  0 . 33 2  4 . 39 7  1 . 5 75

ClimaXfrozen   0 . 08 5  0 . 043  0 . 2 9 7  0 . 360  6 . 688  0 . 8 1 0  1 0 . 739  0 . 1 44   2 . 1 93  0 . 1 83  3 . 1 1 0   0 . 549   2 . 68 1  0 . 342  4 . 389   1 . 5 72

Ta ble  3 :  Performance  of  ClimaX  and  the  baselines  on  downscaling  from  MPI-ESM  ( 5 . 625 ° )  to  ERA5  ( 1 . 40625 ° ) .

Z500  T850  T2m  U 1 0  V 1 0

RMSE  Pearson  Mean  bias  RMSE  Pearson  Mean  bias  RMSE  Pearson  Mean  bias  RMSE  Pearson  Mean  bias  RMSE  Pearson  Mean  bias

ResNet   8 2 5 . 75  0 . 96   − 1 08 . 54  3 . 60  0 . 96  0 . 1 9  2 . 89  0 . 98  0 . 1 4  4 . 0 5  0 . 6 5  0 . 06  4 . 1 1  0 . 45  0 . 09

UNet   8 5 8 . 3 5  0 . 9 5  3 5 . 1 0  3 . 66  0 . 96   − 0 . 34  2 . 9 5  0 . 98  0 . 1 6  4 . 09  0 . 64   − 0 . 06  4 . 1 3  0 . 44  0 . 08

C li-ViT   8 1 1 . 6 1  0 . 96   − 54 . 3 2  3 . 5 8  0 . 9 7   − 0 . 2 9  2 . 80  0 . 99   − 0 . 06  4 . 0 1  0 . 66   − 0 . 08  4 . 0 7  0 . 47  0 . 0 1

ClimaX   80 7 . 43   0 . 96   2 . 70  3 . 49   0 . 9 7   − 0 . 1 1  2 . 79   0 . 99   − 0 . 06  3 . 99  0 . 66  0 . 04  4 . 06   0 . 47   − 0 . 02

bias  closest  to  0  for  all  three  t arget  variables ,  and  performs  similarly  to  the  baselines  in  terms  of  Pearson

correlation .  While  pretrained  to  perform  forecasting,  ClimaX  has  successfully  captured  the  spatial  structure

of  weather  data,  which  helps  in  downstream  tasks  like  downscaling.  Figure  1 0  visualizes  the  downscaled

predictions  of  ClimaX  for  the  three  target  variables .  The  input  is  at  a  much  lower  resolution  and  contains  a

lot  of  bias  compared  to  the  ground  truth .  While  the  prediction  is  missing  some  fine  details ,  it  has  successfully

captured  the  general  structure  of  the  ERA5  data  and  removed  input  biases .

4 . 5 .  S ca l i ng  l aws  a n a l ysi s

Transformers  have  shown  favorable  scaling  properties  for  language  [Kap+ 20 ;  Hof+ 22] ,  vision  [Zha+ 22a] ,  or

even  multi-modal  tasks  [Hen+ 20b ;  Hen+ 2 1 ;  Ree+ 22b] .  That  is ,  their  performance  improves  with  respect  to

data  size  and  model  capacity  given  sufficient  compute .  In  this  section ,  we  study  the  scaling  laws  of  ClimaX

in  weather  forecasting.  Figure  1 1  presents  the  performance  of  ClimaX  as  a  function  of  data  size  and  model

capacity.  The  ��-axis  is  the  pretraining  data  size  measured  in  Gigabytes ,  which  corresponds  to  1  to  5  CMIP6

datasets ,  and  the  ��-axis  shows  the  RMSE  of  ClimaX  on  the  3-day  forecasting  task.  We  compare  four  ClimaX

models  with  different  capacities  by  varying  the  embedding  dimension  from  1 28  to  1 024 .  All  experiments

°

are  conducted  on  the  5 . 62 5   data.  The  error  rate  of  the  two  biggest  models  decreases  consistently  as  we

increase  the  data  and  model  size .  This  highlights  the  unique  ability  of  ClimaX  in  learning  from  diverse  and

heterogeneous  data  sources ,  which  allows  us  to  further  improve  the  performance  by  simply  pretraining  on

more  data.  However ,  the  two  smaller  models  do  not  scale  as  well  as  the  bigger  ones ,  where  increasing  data  size

does  not  gain  much  improvement  or  can  sometimes  hurt  performance .  This  result  shows  that  larger  models

not  only  perform  better  but  are  also  more  data  efficient .

In  addition  to  data  size  and  model  capacity,  data  resolution  is  another  important  scaling  dimension  in  the

context  of  weather  and  climate .  In  many  vision  tasks  such  as  classification ,  understanding  the  general ,

high-level  structure  of  the  image  is  sufficient  to  make  accurate  predictions .  To  model  the  underlying  complex

physical  processes  that  govern  weather  and  climate ,  however ,  it  is  important  for  a  model  to  look  at  fine-grained

details  of  the  input  in  order  to  understand  the  spatial  and  temporal  structure  of  data  as  well  as  the  interactions

between  different  variables .  High-resolution  data  contains  finer  details  and  local  processes  of  weather  conditions

that  are  not  present  in  the  low-resolution  data,  and  thus  provides  stronger  signals  for  training  deep  learning

° °

models .  Figure  1 2  compares  the  performance  of  ClimaX  pretrained  and  finetuned  on  5 . 625   and  1 . 40625

°

data  on  global  forecasting .  Except  for  T2m  at  1  day  and  3  days  lead  times ,  ClimaX   ( 1 . 4062 5 )  consistently

achieves  lower  RMSE  and  higher  AC C  than  the  low-resolution  model .  We  note  that  for  the  high-resolution

data  we  have  to  use  a  larger  patch  size   ( 4  compared  to  2  for  low-resolution  data)  due  to  lack  of  memory  issue .

°

We  can  further  improve  the  performance  of  ClimaX  on  the  1 . 40625   data  by  reducing  the  patch  size ,  as  the

1 7

ClimaX :  A  foundation  model  for  weather  and  climate

# Low- res I n p u t   G ro u n d Tru t h   5 7 5 00 
 D ow n sc a l ed Pred i ct i o n   B i a s

0 
 5 5 0 0 0 
 5 5 0 0 0 
 5 6 0 0 0 
 2 0 0 0

## 05 5 2 5 0 0 
 5 40 0 0 
 0

## Z 5 0 0 0 0 
 5 0 0 0 0 
 5 2 0 0 0 
 2 0 0 0

4 5 0 0 0 
 4 7 5 0 0 
 5 0 0 0 0 
 4 0 0 0

# Low- res I n p u t   G ro u n d Tru t h   Dow n sc a l ed Pred i ct i o n   B i a s

3 4 0 
 3 0 0 
 1 0

05 
 3 2 0 
 2 8 0 
 2 8 0

## 8 0

T 3 0 0 
 2 6 0 
 2 6 0

2 8 0 
 1 0

2 4 0

# Low- res I n p u t   G ro u n d Tru t h   D ow n sc a l ed Pred i ct i o n   B i a s 
 2 0

3 0 0 
 3 0 0

2 8 0 
 1 0

m 
 2 8 0 
 2 8 0

## 2 0

T 2 6 0 
 2 6 0 
 2 6 0 
 1 0

2 4 0

2 4 0 
 2 4 0 
 2 0

2 2 0

Figu re  1 0 :  Example  visualizations  of  downscaled  prediction  of  key  variables  by  ClimaX .

Z5 0 0  [ m 2
/s2
]
 T2 m  [ K]
 T85 0  [ K]
 U 1 0  [ m/s]

2 . 2
 2 . 8

)s 
 350
 2 . 0

ay 2 . 0
 2 . 6

d- 3 0 0
 1 . 8

(3  1 . 8
 2 . 4

SE 1 . 6

M 25 0
 1 . 6
 2 . 2

R 1 . 4

2 0 0
 1 . 4
 2 . 0

5000  1 0000
 5000  1 0000
 5000  1 0000
 5000  1 0000

D ata  s i ze  [G ]

Cl i maX  D= 1 28  Cl i maX  D=256  Cl i maX  D=5 1 2  Cl i maX  D= 1 024

F igu re  1 1 :  Error  on  ERA5  3-day  forecasting  for  different  variables  with  respect  to  CMIP6  5 . 625 °  data  seen

during  pre-training.  Bigger  models  are  more  sample  efficient .

model  is  able  to  capture  better  det ails .

4 . 6 .  A b l atio n  st u d ies

In  the  main  forecasting  results ,  we  finetune  a  separate  ClimaX  model  for  each  target  variable  at  each  lead

time ,  as  we  found  this  protocol  led  to  the  best  performance .  However ,  this  can  be  computationally  expensive ,

as  finetuning  cost  scales  linearly  with  respect  to  the  number  of  target  variables  and  lead  times .  In  this  section ,

we  consider  different  finetuning  alternatives  to  investigate  the  trade-off  between  computation  and  performance .

4 . 6 . 1 .  S hou ld  we  fi netu ne  C l i ma X  for  each  varia b le  separately  or  a l l  at  on ce?

Instead  of  finetuning  ClimaX  for  each  target  variable  separately,  we  could  alternatively  finetune  once  to

predict  all  variables  in  the  input  simultaneously,  which  we  denote  as  ClimaX-all-vars .  Figure  1 3  shows  that

ClimaX-all-vars  achieves  comparable  performance  to  ClimaX  in  most  of  the  tasks  and  only  underperforms  for

forecasting  T2m .  This  suggests  that  with  a  limited  budget ,  one  can  finetune  ClimaX  to  predict  all  target

variables  at  the  same  time  without  losing  much  performance .

4 . 6 . 2 .  S hou l d  we  do  iterative  forecast  or  d i rect  forecast?

To  avoid  finetuning  a  different  model  for  each  lead  time ,  we  can  finetune  ClimaX  to  make  predictions  at

a  short  horizon  such  as  6  hours ,  and  roll  out  the  predictions  during  inference  to  make  forecasts  at  longer

1 8

ClimaX :  A  foundation  model  for  weather  and  climate

Z500
 T850
 T2 M
 U 1 0
 Z500
 T850
 T2 M
 U 1 0

1 . 0

2 . 5
 2 . 0
 3 . 0

5 0 0
 0 . 8
 0 . 8
 0 . 8
 0 . 8

) 
 40 0
 2 . 0
 1 . 5
 2 . 5
 )

(←  
 2 . 0
 → 0 . 6
 0 . 6
 0 . 6
 0 . 6

E 3 0 0
 1 . 5
 (

SM 1 . 0
 1 . 5
 CC 0 . 4
 0 . 4
 0 . 4
 0 . 4

R 200
 1 . 0
 A

1 . 0

1 0 0
 0 . 5
 0 . 5
 0 . 2
 0 . 2
 0 . 2
 0 . 2

0 . 5

0
 0 . 0
 0 . 0
 0 . 0
 0 . 0
 0 . 0
 0 . 0
 0 . 0

1   3  5  7
 1   3  5  7
 1   3  5  7
 1   3  5  7
 1   3  5  7
 1   3  5  7
 1   3  5  7
 1   3  5  7

Leadti m e  [d ays]
 Leadti m e  [d ays]

C l i m aX  (5 . 625 °)  C l i m aX  ( 1 . 40625 °)
 C l i m aX  (5 . 625 °)  C l i m aX  ( 1 . 40625 °)

°

F igu re  1 2 :  Scaling  performance  with  respect  to  data  resolution .  Despite  a  larger  patch  size ,  ClimaX   ( 1 . 4062 5 )

achieves  consistently  better  performance  than  the  low-resolution  model  on  almost  all  tasks ,  except  for  T2m

forecast  at  1  day  and  3  days  lead  times .

horizons .  We  call  this  model  ClimaX-iter ,  where  iter  stands  for  iterative  prediction   [Ras+ 20] .  We  note  that

in  order  to  roll  out  more  than  one  step ,  ClimaX-iter  must  predict  for  all  input  variables ,  or  in  other  words .

This  provides  the  benefit  of  finetuning  a  single  model  that  can  predict  for  any  target  variable  at  any  lead

time .  Figure  1 3  shows  that  ClimaX-iter  works  reasonably  well  up  to  1-day  prediction ,  but  the  performance

degrades  significantly  at  longer  lead  times .  This  is  not  surprising ,  because  ClimaX-iter  is  not  finetuned  to

predict  multiple  steps  into  the  future ,  leading  to  quick  error  accumulation .  One  can  employ  a  multi-step

obj ective  for  finetuning  as  in  Pathak ,  Subramanian ,  et  al .   [Pat + 2 2]  to  achieve  better  results .

4 . 6 . 3 .  Ca n  we  fi netu ne  C l i ma X  to  work  for  a l l  lead  ti mes?

Another  way  to  avoid  finetuning  for  each  lead  time  separately  is  to  finetune  a  lead-time-conditioned  model .

Specifically,  during  finetuning,  we  randomize  the  lead  time  from  6  hours  to  7  days ,  resembling  the  pretraining

setting .  Note  that  unlike  ClimaX-iter ,  we  still  have  to  finetune  a  separate  model  for  each  target  variable .

We  call  this  model  ClimaX-cont ,  wherein  cont  stands  for  continuous,  a  standard  term  used  in  previous

works  [Ras+ 20] .  Figure  1 3  shows  that  ClimaX-cont  performs  competitively  on  6-hour  to  7-day  forecasting,

but  fails  to  extrapolate  to  2  weeks  and  1  month  lead  times  that  are  unseen  during  training .  One  can  also

randomize  the  lead  time  from  6  hours  to  1  month ,  but  that  means  the  model  sees  much  fewer  data  points  for

each  target  lead  time ,  potentially  hurting  the  performance .

The  cost  for  finetuning  each  set  of  weights  is  a  constant  �� ,  which  is  about  1 5  hours  on  an  8  ×   V 1 00�� .  Among

different  finetuning  protocols ,  ClimaX  is  the  most  expensive ,  whose  total  cost  is  �� × #������������������ × #��������_���������� ,

scaling  linearly  with  the  number  of  target  variables  and  lead  times .  Following  ClimaX  are  ClimaX-all-vars  and

ClimaX-cont ,  whose  total  costs  are  ��  ×   #�� ������_ ����������  and  ��  ×   #������������������ ,  respectively.  Finally,  ClimaX-iter

is  the  cheapest  finetuning  protocol ,  where  we  only  have  to  finetune  a  single  model  that  works  for  all  target

variables  and  at  all  lead  times .  The  performance  is  proportional  to  the  computational  cost ,  as  ClimaX  is  the

best  performing  model ,  while  ClimaX-iter  is  the  worst .

# 5 .   D i sc u ssi o n  a n d  Fut u re  Work

The  scaling  of  datasets ,  model  architectures ,  and  computation  has  resulted  in  a  transformative  impact  in

various  subdisciplines  of  artificial  intelligence ,  from  natural  language  and  speech  processing  to  computer

vision ,  as  well  as  scientific  applications  in  biology  and  chemistry.  In  particular ,  it  has  led  to  the  emergence  of

general-purpose  foundation  models  that  are  trained  on  large  datasets  and  compute  clusters ,  and  can  be  easily

adapted  to  a  variety  of  downstream  tasks  efficiently,  both  in  terms  of  compute  and  data  supervision .  Our  work

1 9

ClimaX :  A  foundation  model  for  weather  and  climate

Z5 0 0  [ m 2
/s2
]
 T2 m  [ K]
 T85 0  [ K]
 U 1 0  [ m/s]

1 2 0 0
 6

4 
 5

1 0 0 0
 5

4

8 0 0
 3
 4

E

S 6 0 0
 3
 3

MR tre 
 2

40 0
 bte  2
 2

isr  1

2 0 0 
 e 1 
 1

low

0 
 0 
 0 
 0

1 . 0 
 re 
 1 . 0 
 1 . 0 
 1 . 0

tte

b

0 . 8
 isr  0 . 8
 0 . 8
 0 . 8

he

C 
 ihg

C 0 . 6
 0 . 6
 0 . 6
 0 . 6

A

0 . 4
 0 . 4
 0 . 4
 0 . 4

0 . 2
 0 . 2
 0 . 2
 0 . 2

1   3   5  7  1 0  1 4
 1   3   5  7  1 0  1 4
 1   3   5  7  1 0  1 4
 1   3   5  7  1 0  1 4

Leadti m e  [d ays]

C l i m aX  (5 . 625 °)  C l i m aX-co nt  (5 . 625 °)  C l i m aX- ite r  (5 . 625 °)  C l i m aX-al l -vars  (5 . 625 °)

Figu re  1 3 :  Performance  of  ClimaX  and  its  variations  on  weather  forecasting.  ClimaX-cont  is  a  lead-time

conditioned  model  that  we  finetune  to  make  predictions  at  6  hours  to  7  days .  ClimaX-iter  forecasts  at  a

6-hour  lead  time  and  rolls  out  the  predictions  to  forecast  at  longer  horizons .  ClimaX-all-vars  predicts  the

future  conditions  of  all  variables  in  the  input  at  particular  lead-times .

represents  a  pioneering  effort  to  enable  such  broad  scaling  and  generality  in  data-driven  models  for  weather

and  climate .  This  approach  goes  beyond  the  limitations  of  both  traditional  numerical  modeling  and  existing

data-driven  forecasting  methods .  Unlike  ClimaX ,  numerical  models  scale  only  in  terms  of  computation  and

not  in  terms  of  dataset  size ,  whereas  existing  data-driven  models  are  typically  limited  to  specific  tasks  and

lack  general-purpose  applicability  across  a  wide  range  of  tasks .

In  addition  to  traditional  considerations  in  language  and  vision ,  foundation  models  like  ClimaX  open  up

new  opportunities  for  scaling  through  the  use  of  simulation  datasets  and  grid  resolutions .  To  simplify  our

approach ,  we  chose  to  use  pretraining  datasets  that  include  standard  variables  that  have  been  benchmarked

in  previous  research  on  data-driven  forecasting   [Ras+ 20 ;  Pat + 22] .  Additionally,  we  avoided  datasets  that

simulate  future  scenarios  under  different  forcings  to  prevent  any  potential  leakage  for  the  climate  proj ection

task.  Future  research  could  explore  incorporating  both  observational  and  simulated  datasets  that  include  a

wider  range  of  climate  variables ,  higher  spatiotemporal  resolutions ,  and  even  extend  into  future  scenarios .

Further ,  we  showed  that  resolution  plays  a  crucial  role  in  scaling  of  ClimaX .  Due  to  our  compute  restrictions ,

we  trained  ClimaX  on  low  to  moderate  resolutions .  Nevertheless ,  our  empirical  trends  suggest  that  scaling  to

°

higher  resolutions   ( 0 . 2 5 )  is  likely  to  lead  to  even  b etter  results .

Scaling  efforts  in  the  future  can  benefit  from  better  sequence  modeling  architectures ,  especially  those  designed

for  multimodal  spatiotemporal  inputs .  As  we  saw  in  ClimaX ,  the  number  of  channels  for  climate  datasets  is

much  greater  than  those  handled  for  standard  multimodal  settings  (e . g. ,  audio-video ,  vision-language  models) .

Moreover ,  in  practice ,  there  is  also  a  significant  range  of  resolutions  across  different  climate  datasets .  This

heterogeneity  drastically  increases  the  raw  length  of  input  sequences  for  standard  architectures  such  as  ViT .

In  the  future ,  we  believe  that  investigating  single  multi-scale  architectures   (e . g . ,   [Fan+ 2 1 ] )  can  potentially  aid

in  scaling  to  such  diverse  multi-resolution  and  multi-modal  datasets  by  learning  to  infer  features  relevant  to

atmospheric  phenomena  at  increasing  spatial  resolutions .

20

ClimaX :  A  foundation  model  for  weather  and  climate

In  conclusion ,  we  believe  that  the  generality  of  our  approach  has  potential  applications  beyond  the  tasks

considered  in  this  work .  It  would  be  interesting  to  explore  the  generalization  of  a  pretrained  ClimaX  backbone

to  other  Earth  systems  science  tasks ,  such  as  predicting  extreme  weather  events   [Mir+ 1 9 ;  Sil+ 1 7]  and  assessing

anthropogenic  contributions  to  climate  change   [Ros+08 ;  HT 1 3] ,  as  well  as  broader  domains  that  are  closely

tied  to  weather  and  climate  conditions ,  such  as  agriculture ,  demography,  and  actuarial  sciences .

# Acknowledgments

We  would  like  to  thank  ECMWF  for  enabling  this  line  of  research  with  accessible  public  datasets ,  contributors

of  WebPlotDigitizer   [Roh22]  for  making  it  easier  to  build  Tables  1 0  and  1 1 ,  and  numerous  other  open-source

libraries ,  notably  numpy   [Har+ 20]  and  PyTorch   [Pas+ 1 9b] .  Some  icons  in  Fig.  1  by  Freepik ,  smalllikeart ,  and

GOWI  from  f lat i c on . c om.

Referen ces

[Ado 1 4]   IP C C  Adopted . “  Climate  change  20 1 4  synthesis  report .”  In :  IPCC:   Geneva,  Szwitzerland  ( 20 1 4) .

[Arc+20]  Troy  Arcomano ,  Istvan  Szunyogh,  Jaideep  Pathak,  Alexander  Wikner ,  Brian  R  Hunt ,  and

Edward  Ott . “  A  machine  learning-based  global  atmospheric  forecast  model .”  In :  Geophysical

Res earch  Letters  47. 9  ( 2020) ,  e2020GL087776 .

[Bal+ 22]  V  Balaj i ,  Fleur  Couvreux,  Julie  Deshayes ,  Jacques  Gautrais ,  Frédéric  Hourdin ,  and  Catherine

Rio . “  Are  general  circulation  models  obsolete?”  In :  Proceedings   of  the  National  A cademy   of

Sciences  1 1 9 . 47   ( 202 2 ) ,  e2 2020 75 1 1 9 .

[Bau+20]  Peter  Bauer ,  Tiago  Quintino ,  Nils  Wedi ,  Antonio  Bonanni ,  Marcin  Chrust ,  Willem  Deconinck,

Michail  Diamantakis ,  Peter  Düben,  Stephen  English,  Johannes  Flemming,  et  al.  The   ecmwf

scalability  programme:  Progress  and  plans.  European  Centre  for  Medium  Range  Weather  Forecasts ,

2 0 2 0 .

[BGS20]  Lea  Beusch,  Lukas  Gudmundsson,  and  Sonia  I  Seneviratne . “  Emulating  Earth  system  model

temperatures  with  MESMER:  from  global  mean  temperature  traj ectories  to  grid-point-level

realizations  on  land .”  In :  Earth  System  Dynamics  1 1 . 1   ( 2020) ,  pp .  1 39–1 59 .

[Bi+ 22]  Kaifeng  Bi ,  Lingxi  Xie ,  Hengheng  Zhang,  Xin  Chen ,  Xiaotao  Gu ,  and  Qi  Tian . “  Pangu-Weather :

A  3D  High-Resolution  Model  for  Fast  and  Accurate  Global  Weather  Forecast .”  In:  arXiv  preprint

arXiv:221 1 . 02556  ( 202 2 ) .

[BMMG20]  Jorge  Baño-Medina,  Rodrigo  Manzanas ,  and  José  Manuel  Gutiérrez . “  Configuration  and  inter

comparison  of  deep  learning  neural  models  for  statistical  downscaling.”  In :  Geoscientific  Model

Development  1 3 . 4   ( 2020) ,  pp .  2 1 09–2 1 24 .

[Bom+2 1]  Rishi  Bommasani ,  Drew  A .  Hudson ,  et  al . “  On  the  Opportunities  and  Risks  of  Foundation

Models .”  In :  A rXiv  ( 202 1 ) .  u r  l :  http s : / / crfm . st anf ord . edu/ as s et s /report . pdf .

[Bou+ 1 0]  Philippe  Bougeault ,  Zoltan  Toth ,  Craig  Bishop ,  Barbara  Brown ,  David  Burridge ,  De  Hui  Chen ,

Beth  Ebert ,  Manuel  Fuentes ,  Thomas  M  Hamill,  Ken  Mylne ,  et  al. “  The  THORPEX  interactive

grand  global  ensemble .”  In :  Bulletin   of  the  A merican  Meteorological  Society  9 1 . 8   ( 20 1 0) ,  pp .  1 059–

1 0 72 .

[Bra+22]  Johannes  Brandstetter ,  Rianne  van  den  Berg,  Max  Welling,  and  Jayesh  K  Gupta. “  Clifford

Neural  Layers  for  PDE  Modeling.”  In :  arXiv  preprint   arXiv:2209. 04 934  ( 2022 ) .

[Bro+20]  Tom  Brown,  Benj amin  Mann,  Nick  Ryder ,  Melanie  Subbiah,  Jared  D  Kaplan,  Prafulla  Dhariwal,

Arvind  Neelakantan,  Pranav  Shyam,  Girish  Sastry,  Amanda  Askell,  et  al. “  Language  models  are

few-shot  learners .”  In :  A dvances  in  neural  information  processing  systems  33   ( 2020) ,  pp .  1 877–

1 9 0 1 .

2 1

ClimaX :  A  foundation  model  for  weather  and  climate

[BTB 1 5]  Peter  Bauer ,  Alan  Thorpe ,  and  Gilbert  Brunet . “  The  quiet  revolution  of  numerical  weather

prediction . ”  In :  Nature  5 2 5 . 756 7   ( 20 1 5 ) ,  pp .  47–5 5 .

[BWW22]  Johannes  Brandstetter ,  Daniel  Worrall,  and  Max  Welling. “  Message  Passing  Neural  PDE  Solvers .”

In :  arXiv  preprint   arXiv:2202. 03376  ( 2022 ) .

[Cho+22]  Aakanksha  Chowdhery,  Sharan  Narang,  Jacob  Devlin,  Maarten  Bosma,  Gaurav  Mishra,  Adam

Roberts ,  Paul  Barham,  Hyung  Won  Chung,  Charles  Sutton,  Sebastian  Gehrmann,  et  al. “  PaLM :

Scaling  language  modeling  with  pathways .”  In :  arXiv  preprint   arXiv:2204 . 0231 1  ( 2022 ) .

[Con+22]  Yezhen  Cong,  Samar  Khanna,  Chenlin  Meng,  Patrick  Liu,  Erik  Rozi ,  Yutong  He ,  Marshall

Burke ,  David  B  Lobell ,  and  Stefano  Ermon . “  Satmae :  Pre-training  transformers  for  temporal

and  multi-spectral  satellite  imagery .”  In :  arXiv  preprint   arXiv:2207. 08051  ( 2022 ) .

[DB 1 8]   Peter  D  Dueben  and  Peter  Bauer . “  Challenges  and  design  choices  for  global  weather  and  climate

models  based  on  machine  learning.”  In :  Geoscientific  Model  Development  1 1 . 1 0  ( 20 1 8) ,  pp .  3999–

40 0 9 .

[Dev+ 1 8]  Jacob  Devlin ,  Ming-Wei  Chang,  Kenton  Lee ,  and  Kristina  Toutanova. “  Bert :  Pre-training  of  deep

bidirectional  transformers  for  language  understanding.”  In :  arXiv  preprint   arXiv: 1 81 0. 04 805

( 2 0 1 8 ) .

[Dos+20]  Alexey  Dosovitskiy,  Lucas  Beyer ,  Alexander  Kolesnikov,  Dirk  Weissenborn,  Xiaohua  Zhai ,

Thomas  Unterthiner ,  Mostafa  Dehghani ,  Matthias  Minderer ,  Georg  Heigold ,  Sylvain  Gelly,  et  al .

“ An  image  is  worth  1 6x1 6  words :  Transformers  for  image  recognition  at  scale .”  In :  arXiv  preprint

arXiv:201 0. 1 1 929  ( 2020) .

[Ern2 1]  Lukas  Ernst . “  Structured  Attention  Transformers  on  Weather  Prediction.”  MA  thesis .  ETH

Zurich ,  Scalable  Parallel  Computing  Laboratory ,  202 1 .

[Eyr+ 1 6]   Veronika  Eyring,  Sandrine  Bony,  Gerald  A  Meehl ,  Catherine  A  Senior ,  Bj orn  Stevens ,  Ronald  J

Stouffer ,  and  Karl  E  Taylor . “  Overview  of  the  Coupled  Model  Intercomparison  Proj ect  Phase

6  ( CMIP6)  experimental  design  and  organization .”  In :  Geoscientific  Model  Development  9 . 5

( 2 0 1 6 ) ,  pp .   1 93 7– 1 9 5 8 .

[Fan+2 1]  Haoqi  Fan,  Bo  Xiong,  Karttikeya  Mangalam,  Yanghao  Li ,  Zhicheng  Yan,  Jitendra  Malik,  and

Christoph  Feichtenhofer . “  Multiscale  vision  transformers .”  In:  Proceedings   of  the  IEEE/C VF

International   Conference   on   Computer   Vision.  202 1 ,  pp .  6824–6835 .

[GB22]  Jayesh  K  Gupta  and  Johannes  Brandstetter . “  Towards  Multi-spatiotemporal-scale  Generalized

PDE  Modeling.”  In :  arXiv  preprint   arXiv:2209. 1 561 6  ( 2022 ) .

[GKH 1 5]   Aditya  Grover ,  Ashish  Kapoor ,  and  Eric  Horvitz . “  A  deep  hybrid  model  for  weather  forecasting.”

In:  Proceedings   of  the  21 th  A CM  SIGKDD  international   conference   on  know ledge  discovery  and

data  mining.  20 1 5 ,  pp .  3 79–386 .

[Gro22]   Aditya  Grover . “  Rethinking  Machine  Learning  for  Climate  Science :  A  Dataset  Perspective .”  In :

A A A I  Symposium   on   The  Role   of  A I  in  Responding  to   Climate   Challenges.  2022 .

[Har+ 20]   Charles  R.  Harris ,  K .  Jarrod  Millman ,  Stéfan  J .  van  der  Walt ,  Ralf  Gommers ,  Pauli  Virtanen ,

David  Cournapeau ,  Eric  Wieser ,  Julian  Taylor ,  Sebastian  Berg,  Nathaniel  J .  Smith ,  Robert  Kern ,

Matti  Picus ,  Stephan  Hoyer ,  Marten  H .  van  Kerkwij k,  Matthew  Brett ,  Allan  Haldane ,  Jaime

Fernández  del  Río ,  Mark  Wiebe ,  Pearu  Peterson,  Pierre  Gérard-Marchant ,  Kevin  Sheppard ,

Tyler  Reddy,  Warren  Weckesser ,  Hameer  Abbasi ,  Christoph  Gohlke ,  and  Travis  E .  Oliphant .

“ Array  programming  with  NumPy .”  In :  Nature  585 . 7825  ( Sept .  2020) ,  pp .  35 7–362 .  d o  i :

1 0 . 1 038/ s4 1 586 - 020 - 2649 - 2 .  u r  l :  https : / /do i . org/ 1 0 . 1 038/ s4 1 586 - 020 - 2649 - 2 .

[He+ 1 6]  Kaiming  He ,  Xiangyu  Zhang,  Shaoqing  Ren ,  and  Jian  Sun . “  Deep  residual  learning  for  image

recognition .”  In :  Proceedings   of  the  IEEE   conference   on   computer  vision   and  pattern  recognition.

20 1 6 ,  pp .  770–778 .

22

ClimaX :  A  foundation  model  for  weather  and  climate

[He+ 22]  Kaiming  He ,  Xinlei  Chen ,  Saining  Xie ,  Yanghao  Li ,  Piotr  Dollár ,  and  Ross  Girshick. “  Masked

autoencoders  are  scalable  vision  learners .”  In:  IEEE/C VF   Conference   on   Computer   Vision  and

Pattern  Recognition   (C VPR) .  2022 ,  pp .  1 6000–1 6009 .

[Hen+20a]  Dan  Hendrycks ,  Xiaoyuan  Liu,  Eric  Wallace ,  Adam  Dziedzic ,  Rishabh  Krishnan,  and  Dawn  Song.

“ Pretrained  Transformers  Improve  Out-of-Distribution  Robustness .”  In:  Proceedings   of  the  58th

A nnual  Meeting   of  the  A ssociation f  or   Computational  Linguistics.  2020 ,  pp .  2 744–2 75 1 .

[Hen+20b]  Tom  Henighan,  Jared  Kaplan,  Mor  Katz ,  Mark  Chen,  Christopher  Hesse ,  Jacob  Jackson,  Heewoo

Jun ,  Tom  B  Brown ,  Prafulla  Dhariwal ,  Scott  Gray ,  et  al . “  Scaling  laws  for  autoregressive

generative  modeling.”  In :  arXiv  preprint   arXiv:201 0. 14 701  ( 2020) .

[Hen+2 1]  Lisa  Anne  Hendricks ,  John  Mellor ,  Rosalia  Schneider ,  Jean-Baptiste  Alayrac ,  and  Aida  Ne

matzadeh . “  Decoupling  the  Role  of  Data,  Attention ,  and  Losses  in  Multimodal  Transformers .”

In :  Transactions   of  the  A ss ociation f  or   Computational  Linguistics  9   ( 202 1 ) ,  pp .  5 70–585 .

[Her+ 1 8]   H  Hersbach ,  B  Bell ,  P  Berrisford ,  G  Biavati ,  A  Horányi ,  J  Muñoz  Sabater ,  J  Nicolas ,  C  Peubey ,

R  Radu ,  I  Rozum ,  et  al . “  ERA5  hourly  data  on  single  levels  from  1 979  to  present .”  In :  Copernicus

Climate   Change  Service   (C3S)   Climate  Data  Store   (CDS)  1 0  ( 20 1 8) .

[Her+20]  Hans  Hersbach ,  Bill  Bell ,  Paul  Berrisford ,  Shoj i  Hirahara,  András  Horányi ,  Joaquín  Muñoz

Sabater ,  Julien  Nicolas ,  Carole  Peubey ,  Raluca  Radu ,  Dinand  Schepers ,  et  al . “  The  ERA5

global  reanalysis .”  In :  Quarterly  Journal   of  the  Royal  Meteorological  Society  1 46 . 730   ( 2020) ,

pp .  1 999–2049 .

[HH 1 7]  Stephan  Hoyer  and  Joe  Hamman . “  xarray:  N-D  labeled  Arrays  and  Datasets  in  Python .”  In :

Journal   of   Open  Res earch  Software  5 . 1   (Apr .  20 1 7) ,  p .  1 0 .  d o  i :  1 0 . 5 334/ j or s . 1 48 .

[Hof+22]  Jordan  Hoffmann,  Sebastian  Borgeaud ,  Arthur  Mensch,  Elena  Buchatskaya,  Trevor  Cai ,  Eliza

Rutherford ,  Diego  de  Las  Casas ,  Lisa  Anne  Hendricks ,  Johannes  Welbl ,  Aidan  Clark,  et  al .

“ Training  Compute-Optimal  Large  Language  Models .”  In:  arXiv  preprint  arXiv:2203. 1 5556

( 2 0 2 2 ) .

[Höh+20]  Kevin  Höhlein,  Michael  Kern,  Timothy  Hewson,  and  Rüdiger  Westermann. “  A  comparative

study  of  convolutional  neural  network  models  for  wind  field  downscaling.”  In :  Meteorological

App lications  2 7 . 6   ( 2020 ) ,  e 1 96 1 .

[HT 1 3]  Mikael  Höök  and  Xu  Tang. “  Depletion  of  fossil  fuels  and  anthropogenic  climate  change—A

review . ”  In :  Energy  po licy  5 2   ( 20 1 3 ) ,  pp .  79 7–809 .

[Hua+ 1 6]  Gao  Huang,  Yu  Sun,  Zhuang  Liu,  Daniel  Sedra,  and  Kilian  Q  Weinberger . “  Deep  networks  with

stochastic  depth .”  In :  European   conference   on   computer  vision.  Springer .  20 1 6 ,  pp .  646–66 1 .

[Hun+ 1 9]  Chris  Huntingford ,  Elizabeth  S  Jeffers ,  Michael  B  Bonsall,  Hannah  M  Christensen,  Thomas  Lees ,

and  Hui  Yang. “  Machine  learning  and  artificial  intelligence  to  aid  climate  change  research  and

preparedness .”  In :  Environmental  Res earch  Letters  1 4 . 1 2   ( 20 1 9 ) ,  p .  1 24007 .

[Hur+ 1 3]  James  W  Hurrell ,  Marika  M  Holland ,  Peter  R  Gent ,  Steven  Ghan ,  Jennifer  E  Kay,  Paul  J

Kushner ,  J-F  Lamarque ,  William  G  Large ,  D  Lawrence ,  Keith  Lindsay,  et  al. “  The  community

earth  system  model :  a  framework  for  collaborative  research .”  In :  Bulletin   of  the  A merican

Meteoro logical  Society  94 . 9   ( 20 1 3 ) ,  pp .  1 339–1 360 .

[Hwa+ 1 9]  Jessica  Hwang,  Paulo  Orenstein,  Judah  Cohen,  Karl  Pfeiffer ,  and  Lester  Mackey. “  Improving

subseasonal  forecasting  in  the  western  US  with  machine  learning.”  In :  Proceedings   of  the  25th

A CM  SIGKDD  International   Conference  on  Knowledge  Discovery   &  Data  Mining.  20 1 9 ,  pp .  2325–

2 3 3 5 .

[Kal03]  Eugenia  Kalnay.  A tmospheric  modeling,   data   assimilation   and  predictability.  Cambridge  univer

sity  press ,  2 00 3 .

[Kap+20]  Jared  Kaplan,  Sam  McCandlish,  Tom  Henighan,  Tom  B  Brown,  Benj amin  Chess ,  Rewon  Child,

Scott  Gray,  Alec  Radford ,  Jeffrey  Wu ,  and  Dario  Amodei . “  Scaling  laws  for  neural  language

models .”  In :  arXiv  preprint   arXiv:2001 . 08361  ( 2020) .

23

ClimaX :  A  foundation  model  for  weather  and  climate

[Kas+2 1]  K  Kashinath ,  M  Mustafa,  A  Albert ,  JL  Wu ,  C  Jiang,  S  Esmaeilzadeh ,  K  Azizzadenesheli ,  R

Wang,  A  Chattopadhyay ,  A  Singh ,  et  al . “  Physics-informed  machine  learning:  case  studies  for

weather  and  climate  modelling.”  In :  Philosophical   Transactions   of  the  Royal  Society  A  379 . 2 1 94

( 2 0 2 1 ) ,  p .  2 0 2 0009 3 .

[KB 14]  Diederik  P  Kingma  and  Jimmy  Ba. “  Adam :  A  method  for  stochastic  optimization .”  In :  arXiv

preprint   arXiv: 14 1 2. 6980  ( 20 1 4) .

[Kei22]  Ryan  Keisler . “  Forecasting  Global  Weather  with  Graph  Neural  Networks .”  In :  arXiv  preprint

arXiv:2202. 07575  ( 2022 ) .

[Koc+2 1]  Dmitrii  Kochkov,  Jamie  A  Smith,  Ayya  Alieva,  Qing  Wang,  Michael  P  Brenner ,  and  Stephan

Hoyer . “  Machine  learning–accelerated  computational  fluid  dynamics .”  In:  Proceedings   of  the

National  A cademy   of  Sciences  1 1 8 . 2 1   ( 202 1 ) ,  e2 1 0 1 784 1 1 8 .

[Lam+22]  Remi  Lam,  Alvaro  Sanchez-Gonzalez ,  Matthew  Willson,  Peter  Wirnsberger,  Meire  Fortu

nato ,  Alexander  Pritzel ,  Suman  Ravuri ,  Timo  Ewalds ,  Ferran  Alet ,  Zach  Eaton-Rosen ,  et

al . “  GraphCast :  Learning  skillful  medium-range  global  weather  forecasting.”  In :  arXiv  preprint

arXiv:221 2. 1 2 794  ( 202 2 ) .

[LGD20]  Yumin  Liu,  Auroop  R  Ganguly,  and  Jennifer  Dy. “  Climate  downscaling  using  YNet :  A  deep

convolutional  network  with  skip  connections  and  fusion .”  In :  Proceedings   of  the  26th  A CM

SIGKDD  International   Conference  on  Knowledge  Discovery   &  Data  Mining.  2020 ,  pp .  3 145–

3 1 5 3 .

[LH 1 7]  Ilya  Loshchilov  and  Frank  Hutter . “  Decoupled  weight  decay  regularization .”  In :  arXiv  preprint

arXiv: 1 71 1 . 051 01  ( 20 1 7) .

[Li+20]  Zongyi  Li ,  Nikola  Kovachki ,  Kamyar  Azizzadenesheli ,  Burigede  Liu,  Kaushik  Bhattacharya,

Andrew  Stuart ,  and  Anima  Anandkumar . “  Fourier  neural  operator  for  parametric  partial

differential  equations .”  In :  arXiv  preprint   arXiv:201 0. 08895  ( 2020) .

[Liu+ 2 1]   Ze  Liu ,  Yutong  Lin ,  Yue  Cao ,  Han  Hu ,  Yixuan  Wei ,  Zheng  Zhang,  Stephen  Lin ,  and  Baining  Guo .

“ Swin  transformer :  Hierarchical  vision  transformer  using  shifted  windows .”  In :  Proceedings   of

the  IEEE/C VF  International   Conference   on   Computer   Vision.  202 1 ,  pp .  1 00 1 2–1 0022 .

[Liu+ 22]   Ze  Liu ,  Han  Hu ,  Yutong  Lin ,  Zhuliang  Yao ,  Zhenda  Xie ,  Yixuan  Wei ,  Jia  Ning,  Yue  Cao ,  Zheng

Zhang,  Li  Dong,  Furu  Wei ,  and  Baining  Guo . “  Swin  Transformer  V2 :  Scaling  Up  Capacity  and

Resolution.”  In:  International   Conference   on   Computer   Vision  and  Pattern  Recognition   (C VPR) .

2 0 2 2 .

[Lor67]   Edward  Lorenz . “  The  nature  and  theory  of  the  general  circulation  of  the  atmosphere .”  In :  World

meteorological   organization  1 6 1   ( 1 96 7) .

[LS Z 1 5]  Kody  Law,  Andrew  Stuart ,  and  Konstantinos  Zygalakis . “  Data  assimilation.”  In:  Cham,  Switzer

land:  Spring er  2 1 4   ( 2 0 1 5 ) ,  p .  5 2 .

[Lu+2 1]  Lu  Lu,  Pengzhan  Jin,  Guofei  Pang,  Zhongqiang  Zhang,  and  George  Em  Karniadakis . “  Learning

nonlinear  operators  via  DeepONet  based  on  the  universal  approximation  theorem  of  operators .”

In :  Nature  Machine  Intel ligence  3 . 3   ( 202 1 ) ,  pp .  2 1 8–2 29 .

[Lu+22]  Kevin  Lu ,  Aditya  Grover ,  Pieter  Abbeel ,  and  Igor  Mordatch . “  Pretrained  transformers  as

universal  computation  engines .”  In :  A A A I   Conference   on  A rtificial  Intelligence.  2022 .

[Lyn08]   Peter  Lynch . “  The  origins  of  computer  weather  prediction  and  climate  modeling.”  In :  Journal   of

computational  physics  2 2 7 . 7   ( 2008) ,  pp .  343 1–3444 .

[Man+20]  Laura  A  Mansfield ,  Peer  J  Nowack,  Matt  Kasoar ,  Richard  G  Everitt ,  William  J  Collins ,  and

Apostolos  Voulgarakis . “  Predicting  global  patterns  of  long-term  climate  change  from  short-term

simulations  using  machine  learning .”  In :  npj   Climate   and  A tmospheric  Science  3 . 1   ( 2020) ,  pp .  1–

9 .

24

ClimaX :  A  foundation  model  for  weather  and  climate

[Mar+22]  Stratis  Markou,  James  Requeima,  Wessel  P  Bruinsma,  Anna  Vaughan,  and  Richard  E  Turner .

“ Practical  Conditional  Neural  Processes  Via  Tractable  Dependent  Predictions .”  In :  arXiv  preprint

arXiv:2203. 08775  ( 2022 ) .

[MD+2 1]  Valérie  Masson-Delmotte ,  Panmao  Zhai ,  Anna  Pirani ,  Sarah  L  Connors ,  Clotilde  Péan,  So

phie  Berger ,  Nada  Caud ,  Y  Chen ,  L  Goldfarb ,  MI  Gomis ,  et  al . “  Climate  change  202 1 :  the

physical  science  basis .”  In :  Contribution   of  working  group  I  to  the  sixth   ass essment  report   of  the

intergovernmental  panel   on   climate   change  2   ( 202 1 ) .

[Mee+00]   Gerald  A  Meehl ,  George  J  Boer ,  Curt  Covey,  Moj ib  Latif,  and  Ronald  J  Stouffer . “  The  coupled

model  intercomparison  proj ect  ( CMIP ) .”  In :  Bulletin   of  the  A merican  Meteorological  Society

8 1 . 2   ( 2 000 ) ,  pp .  3 1 3–3 1 8 .

[Mir+ 1 9]   Diego  G  Miralles ,  Pierre  Gentine ,  Sonia  I  Seneviratne ,  and  Adriaan  J  Teuling. “  Land–atmospheric

feedbacks  during  droughts  and  heatwaves :  state  of  the  science  and  current  challenges .”  In :  A nnals

of  the  New   York  A cademy   of  Sciences  1 436 . 1   ( 20 1 9 ) ,  pp .  1 9–35 .

[Mou+23a]  Soukayna  Mouatadid ,  Paulo  Orenstein,  Genevieve  Flaspohler ,  Judah  Cohen,  Miruna  Oprescu,

Ernest  Fraenkel,  and  Lester  Mackey. “  Adaptive  bias  correction  for  improved  subseasonal  fore

casting .”  In :  Nature   Communications  1 4 . 1   ( 2023 ) ,  p .  3482 .

[Mou+23b]  Soukayna  Mouatadid ,  Paulo  Orenstein,  Genevieve  Elaine  Flaspohler,  Miruna  Oprescu,  Judah  Co

hen,  Franklyn  Wang,  Sean  Edward  Knight ,  Maria  Geogdzhayeva,  Samuel  James  Levang,  Ernest

Fraenkel,  et  al. “  SubseasonalClimateUSA :  A  Dataset  for  Subseasonal  Forecasting  and  Bench

marking.”  In:  Thirty-seventh   Conference   on  Neural  Information  Processing  Systems  Datasets

and  Benchmarks   Track.  2023 .

[Ore+ 1 9]  Boris  N  Oreshkin,  Dmitri  Carpov,  Nicolas  Chapados ,  and  Yoshua  Bengio . “  N-BEATS :  Neural  ba

sis  expansion  analysis  for  interpretable  time  series  forecasting.”  In :  arXiv  preprint   arXiv: 1 905. 1 04 37

( 2 0 1 9 ) .

[Pas+ 1 9a]  Adam  Paszke ,  Sam  Gross ,  Francisco  Massa,  Adam  Lerer ,  James  Bradbury,  Gregory  Chanan,

Trevor  Killeen,  Zeming  Lin,  Natalia  Gimelshein,  Luca  Antiga,  Alban  Desmaison,  Andreas  Kopf,

Edward  Yang,  Zachary  DeVito ,  Martin  Raison,  Alykhan  Tej ani ,  Sasank  Chilamkurthy,  Benoit

Steiner ,  Lu  Fang,  Junj ie  Bai ,  and  Soumith  Chintala. “  PyTorch :  An  Imperative  Style ,  High

Performance  Deep  Learning  Library.”  In:  Advances  in  Neural  Information  Processing  Systems

(NeurIPS) .  Curran  Associates ,  Inc . ,  20 1 9 ,  pp .  8024–8035 .

[Pas+ 1 9b]  Adam  Paszke ,  Sam  Gross ,  Francisco  Massa,  Adam  Lerer ,  James  Bradbury,  Gregory  Chanan,

Trevor  Killeen ,  Zeming  Lin ,  Natalia  Gimelshein ,  Luca  Antiga,  et  al . “  Pytorch :  An  imperative

style ,  high-performance  deep  learning  library .”  In :  A dvances  in  neural  information  processing

systems  3 2   ( 2 0 1 9 ) .

[Pat +22]  Jaideep  Pathak,  Shashank  Subramanian,  Peter  Harrington,  Sanj eev  Raj a,  Ashesh  Chattopadhyay,

Morteza  Mardani ,  Thorsten  Kurth ,  David  Hall ,  Zongyi  Li ,  Kamyar  Azizzadenesheli ,  et  al .

“ Fourcastnet :  A  global  data-driven  high-resolution  weather  model  using  adaptive  fourier  neural

operators .”  In :  arXiv  preprint   arXiv:2202. 1 1 214  ( 202 2 ) .

[Phi56]   Norman  A  Phillips . “  The  general  circulation  of  the  atmosphere :  A  numerical  experiment .”  In :

Quarterly  Journal   of  the  Royal  Meteorological  Society  82 . 35 2   ( 1 956 ) ,  pp .  1 23–1 64 .

[Pro+ 1 8]  Liudmila  Prokhorenkova,  Gleb  Gusev,  Aleksandr  Vorobev,  Anna  Veronika  Dorogush,  and

Andrey  Gulin . “  CatBoost :  unbiased  boosting  with  categorical  features .”  In :  A dvances  in  neural

information  processing  systems  3 1   ( 20 1 8) .

[Rad+2 1]  Alec  Radford ,  Jong  Wook  Kim,  Chris  Hallacy,  Aditya  Ramesh,  Gabriel  Goh,  Sandhini  Agarwal,

Girish  Sastry ,  Amanda  Askell ,  Pamela  Mishkin ,  Jack  Clark ,  et  al . “  Learning  transferable  visual

models  from  natural  language  supervision .”  In :  International   Conference   on  Machine  Learning.

PMLR.  202 1 ,  pp .  8748–8763 .

25

ClimaX :  A  foundation  model  for  weather  and  climate

[Ram+22]  Aditya  Ramesh,  Prafulla  Dhariwal,  Alex  Nichol,  Casey  Chu,  and  Mark  Chen. “  Hierarchical

text-conditional  image  generation  with  clip  latents .”  In :  arXiv  preprint   arXiv:2204 . 061 25  ( 2022 ) .

[Ras+20]  Stephan  Rasp ,  Peter  D  Dueben,  Sebastian  Scher ,  Jonathan  A  Weyn,  Soukayna  Mouatadid ,  and

Nils  Thuerey. “  WeatherBench :  a  benchmark  data  set  for  data-driven  weather  forecasting.”  In :

Journal   of  A dvances  in  Modeling  Earth  Systems  1 2 . 1 1  ( 2020) ,  e2020MS002203 .

[Rav+2 1]  Suman  Ravuri ,  Karel  Lenc ,  Matthew  Willson,  Dmitry  Kangin,  Remi  Lam,  Piotr  Mirowski ,  Megan

Fitzsimons ,  Maria  Athanassiadou ,  Sheleem  Kashem ,  Sam  Madge ,  et  al . “  Skilful  precipitation

nowcasting  using  deep  generative  models  of  radar .”  In :  Nature  597 . 7878   ( 202 1 ) ,  pp .  672–677 .

[Ree+ 22a]  Colorado  J  Reed ,  Ritwik  Gupta,  Shufan  Li ,  Sarah  Brockman,  Christopher  Funk,  Brian  Clipp ,  Sal

vatore  Candido ,  Matt  Uyttendaele ,  and  Trevor  Darrell. “  Scale-MAE :  A  Scale-Aware  Masked  Au

toencoder  for  Multiscale  Geospatial  Representation  Learning.”  In :  arXiv  preprint   arXiv:221 2. 14 532

( 2 0 2 2 ) .

[Ree+22b]  Scott  Reed ,  Konrad  Zolna,  Emilio  Parisotto ,  Sergio  Gómez  Colmenarej o ,  Alexander  Novikov,

Gabriel  Barth-maron,  Mai  Giménez ,  Yury  Sulsky,  Jackie  Kay,  Jost  Tobias  Springenberg,  Tom

Eccles ,  Jake  Bruce ,  Ali  Razavi ,  Ashley  Edwards ,  Nicolas  Heess ,  Yutian  Chen ,  Raia  Hadsell ,

Oriol  Vinyals ,  Mahyar  Bordbar ,  and  Nando  de  Freitas . “  A  Generalist  Agent .”  In :  Transactions

on  Machine  Learning  Research  (2022) .  Featured  Certification.  u r  l :  http s : / / openrevi ew . net /

f orum? id= 1 ikK0kHj vj .

[Rei+ 1 9]  Markus  Reichstein,  Gustau  Camps-Valls ,  Bj orn  Stevens ,  Martin  Jung,  Joachim  Denzler ,  Nuno

Carvalhais ,  et  al . “  Deep  learning  and  process  understanding  for  data-driven  Earth  system

science . ”  In :  Nature  566 . 7743   ( 20 1 9 ) ,  pp .  1 95–204 .

[RFB 1 5]  Olaf  Ronneberger ,  Philipp  Fischer ,  and  Thomas  Brox. “  U-Net :  Convolutional  networks  for

biomedical  image  segmentation.”  In:  International   Conference   on  Medical  image   computing  and

computer- assisted  intervention.  Springer .  20 1 5 ,  pp .  234–24 1 .

[Rod+ 1 8]  Eduardo  Rocha  Rodrigues ,  Igor  Oliveira,  Renato  Cunha,  and  Marco  Netto . “  DeepDownscale :  a

deep  learning  strategy  for  high-resolution  weather  forecast .”  In :  201 8  IEEE   14 th  International

Conference   on   e-Science   (e-Science) .  IEEE .  20 1 8 ,  pp .  4 1 5–42 2 .

[Roh22]  Ankit  Rohatgi .  Webplotdigitizer:   Version  4 . 6.  2022 .  u r  l :  http s : / / aut omer i s . i o /WebPl otD igit izer .

[Ros+08]  Cynthia  Rosenzweig,  David  Karoly,  Marta  Vicarelli ,  Peter  Neofotis ,  Qigang  Wu ,  Gino  Casassa,

Annette  Menzel ,  Terry  L  Root ,  Nicole  Estrella,  Bernard  Seguin ,  et  al . “  Attributing  physical  and

biological  impacts  to  anthropogenic  climate  change .”  In :  Nature  453 . 7 1 93   ( 2008) ,  pp .  353–35 7 .

[RRH22]  AR  Ravishankara,  David  A  Randall,  and  James  W  Hurrell. “  Complex  and  yet  predictable :  The

message  of  the  202 1  Nobel  Prize  in  Physics .”  In :  Proceedings   of  the  National  A cademy   of  Sciences

1 1 9 . 2   ( 2 0 2 2 ) ,  e2 1 2 0669 1 1 9 .

[RT2 1]  Stephan  Rasp  and  Nils  Thuerey. “  Data-driven  medium-range  weather  prediction  with  a  resnet

pretrained  on  climate  simulations :  A  new  model  for  weatherbench .”  In :  Journal   of  A dvances  in

Modeling  Earth  Systems  1 3 . 2  ( 202 1 ) ,  e2020MS002405 .

[Sac+ 1 8]  DA  Sachindra,  Khandakar  Ahmed ,  Md  Mamunur  Rashid ,  S  Shahid ,  and  BJC  Perera. “  Statistical

downscaling  of  precipitation  using  machine  learning  techniques .”  In :  A tmospheric  research  2 1 2

( 2 0 1 8 ) ,  pp .  240–2 5 8 .

[Sat04]  Masaki  Satoh .  A tmospheric   circulation   dynamics   and   circulation  models.  Springer  Science  &

Business  Media,  2004 .

[Sch+ 1 7]   Tapio  Schneider ,  Shiwei  Lan ,  Andrew  Stuart ,  and  Joao  Teixeira. “  Earth  system  modeling  2 . 0 :  A

blueprint  for  models  that  learn  from  observations  and  targeted  high-resolution  simulations .”  In :

Geophysical  Res earch  Letters  44 . 24   ( 20 1 7) ,  pp .  1 2–396 .

[Sch 1 8]  Sebastian  Scher . “  Toward  data-driven  weather  and  climate  forecasting:  Approximating  a  simple

general  circulation  model  with  deep  learning .”  In :  Geophysical  Res earch  Letters  45 . 2 2   ( 20 1 8) ,

pp .   1 2–6 1 6 .

26

ClimaX :  A  foundation  model  for  weather  and  climate

[Sch+ 2 1]   Martin  G  Schultz ,  Clara  Betancourt ,  Bing  Gong,  Felix  Kleinert ,  Michael  Langguth ,  Lukas  Hubert

Leufen,  Amirpasha  Mozaffari ,  and  Scarlet  Stadtler . “  Can  deep  learning  beat  numerical  weather

prediction?”  In :  Philos ophical   Trans actions   of  the  Royal  Society  A  3 79 . 2 1 94   ( 202 1 ) ,  p .  2020009 7 .

[Sil+ 1 7]  Jana  Sillmann ,  Thordis  Thorarinsdottir ,  Noel  Keenlyside ,  Nathalie  Schaller ,  Lisa  V  Alexan

der ,  Gabriele  Hegerl ,  Sonia  I  Seneviratne ,  Robert  Vautard ,  Xuebin  Zhang,  and  Francis  W

Zwiers . “  Understanding,  modeling  and  predicting  weather  and  climate  extremes :  Challenges  and

opportunities .”  In :  Weather   and   climate   extremes  1 8   ( 20 1 7) ,  pp .  65–74 .

[SM 1 9]  Sebastian  Scher  and  Gabriele  Messori . “  Weather  and  climate  forecasting  with  neural  networks :

using  general  circulation  models  ( G CMs)  with  different  complexity  as  a  study  ground .”  In :

Geoscientific  Model  Development  1 2 . 7  ( 20 1 9) ,  pp .  2 797–2809 .

[Søn+20]  Casper  Kaae  Sønderby,  Lasse  Espeholt ,  Jonathan  Heek,  Mostafa  Dehghani ,  Avital  Oliver ,  Tim

Salimans ,  Shreya  Agrawal,  Jason  Hickey,  and  Nal  Kalchbrenner . “  MetNet :  A  neural  weather

model  for  precipitation  forecasting.”  In :  arXiv  preprint   arXiv:2003. 1 214 0  ( 2020) .

[Tao+20]  Rohan  Taori ,  Achal  Dave ,  Vaishaal  Shankar ,  Nicholas  Carlini ,  Benj amin  Recht ,  and  Ludwig

Schmidt . “  Measuring  robustness  to  natural  distribution  shifts  in  image  classification .”  In :  A d

vances  in  Neural  Information  Processing  Systems  33  ( 2020) ,  pp .  1 8583–1 8599 .

[TL 1 8]   Sean  J  Taylor  and  Benj amin  Letham . “  Forecasting  at  scale .”  In :  The  A merican  Statistician  72 . 1

( 2 0 1 8 ) ,  pp .  3 7–45 .

[Tou+2 1]  Hugo  Touvron,  Matthieu  Cord ,  Matthij s  Douze ,  Francisco  Massa,  Alexandre  Sablayrolles ,  and

Hervé  Jégou . “  Training  data-efficient  image  transformers  &  distillation  through  attention .”  In :

International   Conference   on  Machine  Learning.  PMLR.  202 1 ,  pp .  1 0347–1 0357.

[TP 1 2]   Sebastian  Thrun  and  Lorien  Pratt .  Learning  to  learn.  Springer  Science  &  Business  Media,  20 1 2 .

[Van+ 1 7]  Thomas  Vandal,  Evan  Kodra,  Sangram  Ganguly,  Andrew  Michaelis ,  Ramakrishna  Nemani,  and

Auroop  R  Ganguly. “  Deepsd :  Generating  high  resolution  climate  change  proj ections  through

single  image  super-resolution .”  In :  Proceedings   of  the  23rd   acm  sigkdd  international   conference

on  know ledge   dis covery   and   data  mining.  20 1 7 ,  pp .  1 663–1 672 .

[Vas+ 1 7]  Ashish  Vaswani ,  Noam  Shazeer ,  Niki  Parmar ,  Jakob  Uszkoreit ,  Llion  Jones ,  Aidan  N  Gomez ,

Łukasz  Kaiser ,  and  Illia  Polosukhin . “  Attention  is  all  you  need .”  In :  A dvances  in  neural  informa

tion  processing  systems  30   ( 20 1 7) .

[Vau+2 1]  Anna  Vaughan,  Will  Tebbutt ,  J  Scott  Hosking,  and  Richard  E  Turner . “  Convolutional  conditional

neural  processes  for  local  climate  downscaling.”  In :  arXiv  preprint   arXiv:21 01 . 07950  ( 202 1 ) .

[Ver+ 22]   Robert  Verkuil ,  Ori  Kabeli ,  Yilun  Du ,  Basile  IM  Wicky ,  Lukas  F  Milles ,  Justas  Dauparas ,  David

Baker ,  Sergey  Ovchinnikov,  Tom  Sercu,  and  Alexander  Rives . “  Language  models  generalize

beyond  natural  proteins .”  In :  bioRxiv  ( 202 2 ) ,  pp .  202 2–1 2 .

[Vit + 2 2]   F .  Vit art ,  A .  W .  Robertson ,  A .  Spring ,  F .  Pinault ,  R.  Roškar ,  W .  C ao ,  S .  Bech ,  A .  Bienkowski ,  N .

C altabiano ,  E .  De  Coning ,  B .  Denis ,  A .  Dirkson ,  J .  Dramsch ,  P.  Dueben ,  J .  Gierschendorf,  H .  S .

Kim ,  K .  Nowak,  D .  Landry ,  L .  Lledó ,  L .  Palma,  S .  Rasp ,  and  S .  Zhou . “  Outcomes  of  the  WMO

Prize  Challenge  to  Improve  Subseasonal  to  Seasonal  Predictions  Using  Artificial  Intelligence .”

In :  Bulletin   of  the  A merican  Meteorological  Society  1 03 . 1 2   (Dec .  202 2 ) ,  E2878–E2886 .  d o  i :

1 0 . 1 1 75 /bams - d- 22 - 0046 . 1 .

[VKG 1 9]  Thomas  Vandal,  Evan  Kodra,  and  Auroop  R  Ganguly. “  Intercomparison  of  machine  learning

methods  for  statistical  downscaling :  the  case  of  daily  and  extreme  precipitation .”  In :  Theoretical

and  Applied   Climatology  1 3 7 . 1   ( 20 1 9 ) ,  pp .  5 5 7–5 70 .

[VR1 8]  Frédéric  Vitart  and  Andrew  W  Robertson . “  The  sub-seasonal  to  seasonal  prediction  proj ect

( S 2 S )  and  the  prediction  of  extreme  events .”  In :  npj   Climate   and  A tmospheric  Science  1 . 1   ( 20 1 8) ,

pp .   1 –7 .

27

ClimaX :  A  foundation  model  for  weather  and  climate

[Wan+22]  Wenhui  Wang,  Hangbo  Bao ,  Li  Dong,  Johan  Bj orck,  Zhiliang  Peng,  Qiang  Liu,  Kriti  Aggarwal,

Owais  Khan  Mohammed ,  Saksham  Singhal,  Subhoj it  Som,  et  al. “  Image  as  a  foreign  language :

Beit  pretraining  for  all  vision  and  vision-language  tasks .”  In :  arXiv  preprint   arXiv:2208. 1 044 2

( 2 0 2 2 ) .

[War 1 0]  Thomas  Tomkins  Warner .  Numerical  weather  and   climate  prediction.  cambridge  university  press ,

2 0 1 0 .

[WDC20]  Jonathan  A  Weyn,  Dale  R  Durran,  and  Rich  Caruana. “  Improving  data-driven  global  weather

prediction  using  deep  convolutional  neural  networks  on  a  cubed  sphere .”  In :  Journal   of  A dvances

in  Modeling  Earth  Systems  1 2 . 9  ( 2020) ,  e2020MS002 1 09 .

[Web+20]  Theodore  Weber ,  Austin  Corotan,  Brian  Hutchinson,  Ben  Kravitz ,  and  Robert  Link. “  Deep

learning  for  creating  surrogate  models  of  precipitation  in  Earth  system  models .”  In :  A tmospheric

Chemistry   and  Physics  20 . 4   ( 2020) ,  pp .  2303–23 1 7 .

[Wed+ 1 5]  NP  Wedi ,  P  Bauer ,  W  Denoninck,  M  Diamantakis ,  M  Hamrud ,  C  Kuhnlein,  S  Malardel,  K

Mogensen,  G  Mozdzynski ,  and  PK  Smolarkiewicz .  The  modelling  infrastructure   of  the  Integrated

Forecasting  System:  Recent  advances  and f  uture  challenges.  European  Centre  for  Medium-Range

Weather  Forecasts ,  20 1 5 .

[Wey+2 1]  Jonathan  A  Weyn,  Dale  R  Durran,  Rich  Caruana,  and  Nathaniel  Cresswell- Clay. “  Sub-seasonal

forecasting  with  a  large  ensemble  of  deep-learning  weather  prediction  models .”  In :  Journal   of

A dvances  in  Modeling  Earth  Systems  1 3 . 7  ( 202 1 ) ,  e202 1 MS002502 .

[Wig19]  Ross  Wightman.  Py Torch  Image  Models.  https : / /github . com/rwightman/pyt orch- image 

mode l s .  20 1 9 .  d o  i :  1 0 . 5 28 1 /z enodo . 44 1 486 1 .

[WP+22]  Duncan  Watson-Parris ,  Yuhan  Rao,  Dirk  Olivié,  Øyvind  Seland,  Peer  Nowack,  Gustau  Camps

Valls ,  Philip  Stier ,  Shahine  Bouabid ,  Maura  Dewey ,  Emilie  Fons ,  et  al . “  ClimateBench  v 1 .  0 :  A

Benchmark  for  Data-Driven  Climate  Proj ections .”  In:  Journal   of  A dvances  in  Modeling  Earth

Systems  1 4 . 1 0  ( 2022 ) ,  e202 1 MS002954 .

[WW97]  Robert  L  Wilby  and  Thomas  ML  Wigley. “  Downscaling  general  circulation  model  output :  a

review  of  methods  and  limitations .”  In :  Progress  in  physical  geography  2 1 . 4   ( 1 997) ,  pp .  530–548 .

[YL20]  Yuan  Yuan  and  Lei  Lin . “  Self-supervised  pretraining  of  transformers  for  satellite  image  time

series  classification .”  In :  IEEE  Journal   of  Selected   Topics  in  Applied  Earth   Obs ervations   and

Remote  Sensing  1 4   ( 2020) ,  pp .  474–487 .

[Yua+2 1]  Lu  Yuan,  Dongdong  Chen,  Yi-Ling  Chen,  Noel  Codella,  Xiyang  Dai ,  Jianfeng  Gao ,  Houdong  Hu,

Xuedong  Huang,  Boxin  Li ,  Chunyuan  Li ,  et  al . “  Florence :  A  new  foundation  model  for  computer

vision .”  In :  arXiv  preprint   arXiv:21 1 1 . 1 14 32  ( 202 1 ) .

[Zha+ 1 9]  Fuqing  Zhang,  Y  Qiang  Sun,  Linus  Magnusson,  Roberto  Buizza,  Shian-Jiann  Lin,  Jan-Huey

Chen ,  and  Kerry  Emanuel . “  What  is  the  predictability  limit  of  midlatitude  weather?”  In :  Journal

of  the  A tmospheric  Sciences  76 . 4   ( 20 1 9 ) ,  pp .  1 077–1 09 1 .

[Zha+22a]  Xiaohua  Zhai ,  Alexander  Kolesnikov,  Neil  Houlsby,  and  Lucas  Beyer . “  Scaling  vision  transform

ers .”  In:  IEEE/C VF   Conference  on   Computer   Vision  and  Pattern  Recognition   (C VPR) .  2022 ,

pp .  1 2 1 04–1 2 1 1 3 .

[Zha+22b]  Chongzhi  Zhang,  Mingyuan  Zhang,  Shanghang  Zhang,  Daisheng  Jin,  Qiang  Zhou,  Zhongang

Cai ,  Haiyu  Zhao ,  Xianglong  Liu ,  and  Ziwei  Liu . “  Delving  deep  into  the  generalization  of  vision

transformers  under  distribution  shifts .”  In:  IEEE/C VF   Conference  on   Computer   Vision  and

Pattern  Recognition   (C VPR) .  2022 ,  pp .  7277–7286 .

[Zho+2 1]  Haoyi  Zhou ,  Shanghang  Zhang,  Jieqi  Peng,  Shuai  Zhang,  Jianxin  Li ,  Hui  Xiong,  and  Wancai

Zhang. “  Informer :  Beyond  efficient  transformer  for  long  sequence  time-series  forecasting.”  In :

Proceedings   of  the  A A A I   conference   on   artificial  intel ligence .  Vol .  35 .  1 2 .  202 1 ,  pp .  1 1 1 06–1 1 1 1 5 .

[Zhu 1 8]   J  Zhuang.  xESMF:   Universal  regridder f  or  geospatial   data.  20 1 8 .

28

ClimaX :  A  foundation  model  for  weather  and  climate

A .   M o d e l

This  section  presents  the  implementation  details  and  hyperparameters  of  ClimaX  and  the  two  CNN  baselines

UNet  and  ResNet .

A . 1 .  C l i ma X

A . 1 . 1 .  I m p lementation  deta i ls

ClimaX  receives  a  tensor  of  shape  ��  ×   ��  ×   ��  and  outputs  a  tensor  of  shape  ��
 ′  ×   ��  ×   �� ,  where  the  number

of  input  and  output  variables  ��  and  ��
 ′
 can  vary  between  different  datasets5
 .  To  do  that ,  we  assume  a  set  ��

that  contains  all  possible  variables  we  could  encounter  during  pretraining  and  finetuning.  Each  variable  in  ��

has  a  separate  token  embedding  layer .

The  variable  tokenization  module  tokenizes  the  input  to  a  sequence  of  ��  ×   ℎ  ×   ��  tokens ,  with  each  token

being  a  vector  of  size  ��
2
 .  After  that ,  for  each  token ,  we  extract  the  corresponding  embedding  layer  that

transforms  the  token  to  a  vector  of  dimension  �� .  Each  embedding  layer  is  a  single  convolution  layer  with

���� _ ��ℎ���������� ��  =  1 ,  ������ _ ��ℎ���������� ��  =  �� ,  ������������ _ ��������  =  ��,  ������������  =  ��.  This  results  in  a  tensor  of  shape

��  ×   ℎ  ×   ��  ×   �� .

To  differentiate  between  tokens  of  different  input  variables ,  we  add  the  sequence  with  a  variable  positional

embedding,  which  is  a  tensor  of  shape  | �� |   ×   �� .  For  each  input  variable ,  we  extract  the  corresponding  variable

positional  embedding  to  add  to  its  tokens .  After  that ,  all  tokens  go  through  the  variable  aggregation  module ,

which  outputs  a  tensor  of  shape  ℎ  ×   ��  ×   �� .

The  tokens  are  then  fed  to  the  attention  layers ,  which  output  a  tensor  of  the  same  shape  ℎ  ×   ��  ×   �� .  The

prediction  head  takes  each  token  of  dimension  ��  and  maps  it  to  a  vector  of  dimension  | �� |   × ��
2
 ,  and  the  output

is  reshaped  to  | �� |   ×   ��  ×   �� .  Finally,  we  extract  predictions  of  ��
 ′
 target  variables  and  compute  the  loss .

A . 1 . 2 .  H yperpara meters

Ta ble  4 :  Default  hyperparameters  of  ClimaX

Hyperparameter  Meaning  Value

��  Default  variables  All  ERA5  variables  in  Table  9

| �� |   Number  of  default  variables  48

°

2  for  5 . 6 2 5

��  Patch  size  °

4  for  1 . 40 6 2 5

��  Embedding  dimension  1 024

Depth  Number  of  ViT  blocks  8

#  heads  Number  of  attention  heads  1 6

Determine  the  hidden  dimension  of

MLP  ratio  4

the  MLP  layer  in  a  ViT  block

Prediction  depth  Number  of  layers  of  the  prediction  head  2

Hidden  dimension  Hidden  dimension  of  the  prediction  head  1 024

Drop  path  For  stochastic  depth   [Hua+ 1 6]   0 . 1

Dropout  Dropout  rate  0 . 1

5 The  spatial  resolution  ��  ×   ��  can  also  vary.  In  that  case ,  we  employ  the  common  practice  of  interpolating  the  positional

embedding,  and  everything  else  remains  the  same  [Dos+ 20 ;  Tou+ 2 1] .

29

ClimaX :  A  foundation  model  for  weather  and  climate

A . 2 .  C N N  Basel i nes

A . 2 . 1 .  Res N et  H yperpara meters

We  use  the  following  hyperparameters  for  ResNet  in  all  of  our  experiments .

Ta ble  5 :  Default  hyperparameters  of  ResNet

Hyperparameter  Meaning  Value

Padding  size  Padding  size  of  each  convolution  layer  1

Kernel  size  Kernel  size  of  each  convolution  layer  3

Stride  Stride  of  each  convolution  layer  1

Hidden  dimension  Number  of  output  channels  of  each  residual  block  1 28

Residual  blocks  Number  of  residual  blocks  28

Dropout  Dropout  rate  0 . 1

A . 2 . 2 .  U N et  H yperpara meters

We  borrow  our  UNet  implementation  from  PDEArena  [GB22] .  We  use  the  following  hyperparameters  for

UNet  in  all  of  our  experiments .

Ta ble  6 :  Default  hyperparameters  of  UNet

Hyperparameter  Meaning  Value

Padding  size  Padding  size  of  each  convolution  layer  1

Kernel  size  Kernel  size  of  each  convolution  layer  3

Stride  Stride  of  each  convolution  layer  1

Determine  the  number  of  output  channels

C hannel  mult iplicat ions  [ 1 ,  2 ,  2 ,  4]

for  Down  and  Up  blocks

Blocks  Number  of  blocks  2

Use  attention  If  use  attention  in  Down  and  Up  blocks  False

Dropout  Dropout  rate  0 . 1

A . 2 . 3 .  Other  i m plementation  deta i ls

Following  the  implementation  of  ResNet  in  Rasp ,  Dueben ,  et  al .  [Ras+ 20] ,  Rasp  and  Thuerey  [RT2 1] ,  and

Ernst   [Ern2 1] ,  we  found  the  following  details  important  for  the  performance  of  both  CNN  baselines :

•  Use  Batch  normalization

•  Use  Leakyrelu  with  a  slope  of  0 . 3  as  the  activation  function

•  Postnorm  instead  of  Prenorm

•  Use  periodic  convolutions  in  the  longitude  direction  but  not  the  latitude  direction .

•  Use  a  kernel  size  of  7  in  the  first  CNN  layer .

B .   Tra i n i n g  d eta i l s

Data  normalizat ion  We  normalized  all  inputs  during  pre-training  as  well  as  fine-tuning.  For  each  variable ,

at  each  pressure  level  (for  atmospheric  variables) ,  we  compute  the  mean  and  standard  deviation  to  normalize

them  to  zero  mean  and  unit  variance .  We  de-normalize  the  predictions  to  get  back  to  the  original  range  before

computing  evaluation  metrics .

30

ClimaX :  A  foundation  model  for  weather  and  climate

Software  and  hardware  stack  We  use  PyTorch  [Pas+ 1 9a] ,  t imm  [Wig1 9] ,  numpy  [Har+20]  and  xarray

[HH 1 7]  to  manage  our  data  and  model  training.  We  used  32GB  NVIDIA  V 1 00  devices  for  training.  For

pretraining  we  distribute  the  batch  across  80  V 1 00s  on  AzureML .  We  leverage  fp 1 6  floating  point  precision  in

our  model .

B . 1 .  P ret ra i n i ng

B . 1 . 1 .  O bj ective

We  use  the  loss  function  in  Equation   ( 1 )  for  pretraining .

B . 1 . 2 .  O pti m ization

We  used  the  AdamW  optimizer  [KB 1 4 ;  LH 1 7]  with  parameters  ( ��1  =  0 . 9 ,  ��2  =  0 . 95 ) .  We  used  weight  decay

of  1 ��  −  5  for  all  parameters  except  for  the  positional  embedding .  We  used  a  learning  rate  of  5 ��  −  4 ,  with  a

linear  warmup  schedule  for  1 0000  steps  ( 5  epochs) ,  followed  by  a  cosine-annealing  schedule  for  1 90000  steps

( 9 5  ep o chs ) .

B . 2 .  F i n et u n i ng

B . 2 . 1 .  O bjective

We  use  lat-weighted  MSE  in  Equation  ( 1 )  for  finetuning  ClimaX  in  temporal  forecasting  and  downscaling

tasks .  In  ClimateBench ,  we  finetune  using  standard  MSE  without  the  weighting  term ,  as  this  led  to  better

results  and  was  suggested  by   [WP + 22] .

B . 2 . 2 .  O pti m ization

For  all  tasks ,  we  used  AdamW  with  parameters   ( ��1  =  0 . 9 ,  ��2  =  0 . 999) .  We  used  weight  decay  of  1 ��  −  5  for

all  parameters  except  for  the  positional  embedding.  We  used  a  linear  warmup  schedule  for  1 0000  steps  ( 5

epochs) ,  followed  by  a  cosine-annealing  schedule  for  90000  steps   (45  epochs) .  The  learning  rate  for  each  task

is  as  follows :

Ta ble  7 :  Learning  rate  for  finetuning  ClimaX  in  different  downstream  tasks

Task  Learning  rate

Weather  forecasting  5��  −  7

Climate  proj ection  5 ��  −  4

Climate  downscaling  5��  −  5

We  used  a  small  learning  rate  for  weather  forecasting  as  the  task  resembles  pretraining.  For  downscaling,

we  used  a  larger  learning  rate ,  as  the  nature  of  the  task  is  different  from  pretraining ,  even  though  the  input

variables  are  similar .  In  climate  proj ection ,  we  needed  to  initialize  new  weights  for  the  embedding  layers  and

prediction  heads ,  and  thus  used  a  similar  learning  rate  to  training  from  scratch .

C .   D a ta sets

C . 1 .  C M I P 6- C l i maX

We  created  CMIP6- ClimaX  for  pretraining  ClimaX ,  which  consists  of  5  datasets  from  the  CMIP6  proj ect .  We

downloaded  the  datasets  from  the  official  CMIP6  search  interface  at  https : / /esgf-data. dkrz . de/search/cmip6-

dkrz / .  These  datasets  share  the  following  attributes :

3 1

ClimaX :  A  foundation  model  for  weather  and  climate

•  Experiment  ID :  historical

•  Table  ID :  6hrPlevPt ,  i . e . ,  6-hourly  dat a  on  pressure  levels .

•  Variant  label :  r 1 i 1 p 1 f1 .  The  variant  label  distinguishes  among  closely  related  simulations  by  a  single

model ,  in  which “  r”  specifies  the  initial  condition , “  i”  specifies  the  observational  dataset  and  initialization

method  used  for  determining  the  initial  condition , “  p”  specifies  the  perturbed  physics  version  of  the

model ,  and “  f”  specifies  the  forcing  index.

All  datasets  have  a  temporal  coverage  from  1 850  to  20 1 5  and  a  temporal  resolution  of  6  hours .  We  chose

these  datasets  as  they  contain  similar  climate  variables  at  similar  vertical  levels  to  ERA5 .  We  note  that  there

are  more  than  5  datasets  from  CMIP6  that  suit  our  selection  criteria,  but  we  were  not  able  to  download

° °

others  due  to  some  issues  on  the  data  servers .  We  regridded  these  datasets  to  5 . 62 5   and  1 . 4062 5   using  the

xesmf  Python  package   [Zhu 1 8]  using  bilinear  interpolation .  We  provide  a  detailed  description  of  these  5  data

sources  and  the  available  variables  we  used  to  construct  CMIP6- ClimaX  in  Table  8 .

°

We  note  that  AWI  and  HAMMO Z  are  not  the  best  data  sources  for  higher  resolution  1 . 40625   training,

°

because  their  original  resolution  at  2 50  km  is  lower  than  1 . 4062 5 ,  which  is  about  1 56  km .  We  wanted  to

use  other  higher-resolution  datasets  but  were  not  able  to  download  them .  We  believe  pretraining  on  other

high-resolution  datasets  would  lead  to  better  performance .

C . 2 .  E RA5

We  use  the  preprocessed  version  of  ERA5  from  WeatherBench  [Ras+20]  for  finetuning  ClimaX.  WeatherBench

was  created  as  a  standard  benchmark  data  and  evaluation  framework  for  comparing  data-driven  weather

° °

forecasting  models .  WeatherBench  regridded  the  original  ERA5  at  0 . 25   to  three  lower  resolutions :  5 . 625 ,

2 . 8 1 25° ,  and  1 .40625° .  See  https : / / c onf luence . e cmwf . int /di splay/CKB/ERA5%3A+dat a+do cument at i on

for  more  details  of  the  raw  ERA5  data.  Table  9  summarizes  the  variables  we  use  for  finetuning  ClimaX .

C . 2 . 1 .  E RA5- N A

We  constructed  ERA5-NA  from  ERA5  to  evaluate  ClimaX  and  the  baselines  on  regional  forecasting.  ERA-NA

has  the  same  set  of  variables  as  in  Table  9 ,  but  only  contains  data  that  belongs  to  the  North  America  region .

To  do  this ,  we  first  identified  the  latitude  and  longitude  range  to  form  a  rectangular  area  that  encapsulates

North  America,  using  the  standard  CORDEX  domains  https : // cordex . org/wp - cont ent /uploads /20 1 2/

1 1 /CORDEX - domain- de s cr ipt i on_ 23 1 0 1 5 . pdf .  For  each  data  sample ,  we  then  extracted  the  spatial  positions

that  fall  into  this  range ,  forming  in  ERA5-NA .

C . 2 . 2 .  E RA- S 2 S

We  built  ERA5- S 2S  from  ERA5  to  serve  as  a  benchmark  dataset  for  sub-seasonal  to  seasonal  prediction .

ERA5- S 2 S  consists  of  two  sub-datasets ,  whose  the  goals  are  to  predict  the  biweekly  average  statistics  of  target

variables  in  weeks  3  and  4 ,  and  weeks  5  and  6 ,  respectively.  The  input  includes  all  variables  in  Table  9 ,  while

the  output  variables  are  are  averaged  over  two  weeks ,  starting  from  the  start  of  week  3   ( 5 )  and  to  the  end  of

week  4   ( 6 ) .

C . 3 .  C l i mate Bench

We  refer  to  Watson-Parris ,  Rao ,  et  al .   [WP + 22]  for  complete  details  of  ClimateBench .

32

ClimaX :  A  foundation  model  for  weather  and  climate

Ta ble  8 :  Resolution  and  variables  of  CMIP6- ClimaX  dataset  used  for  pretraining.  Static  represents  variables

don ’t  depend  on  time ,  Single  represents  surface  variables ,  and  A tmospheric  represents  time-varying  atmospheric

properties  at  the  chosen  altitudes .

Data  Source  Original  resolution

MPI  1 00km

Tai  1 00km

AWI  250km

HAMMOZ  250km

CMCC  100km

Variables

Type  Abbrev.  Levels

Single  t 2m

S ingle  u 1 0

S ingle  v 1 0

Atmospheric  z  5 0 ,  2 5 0 ,  500 ,  600 ,  700 ,  8 5 0 ,  9 2 5

Atmospheric  u  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  v  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  t  5 0 ,  2 5 0 ,  5 00 ,  600 ,  700 ,  8 5 0 ,  9 2 5

Atmospheric  q  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Single  t 2m

Atmospheric  z  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  u  2 50 ,  500 ,  850

Atmospheric  v  2 50 ,  500 ,  850

Atmospheric  t  2 50 ,  500 ,  850

Atmopheric  q  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Single  t 2m

S ingle  u 1 0

S ingle  v 1 0

Atmospheric  z  5 0 ,  2 5 0 ,  500 ,  600 ,  700 ,  8 5 0 ,  9 2 5

Atmospheric  u  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  v  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  t  5 0 ,  2 5 0 ,  5 00 ,  600 ,  700 ,  8 5 0 ,  9 2 5

Atmospheric  q  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Single  t 2m

S ingle  u 1 0

S ingle  v 1 0

Atmospheric  z  5 0 ,  2 5 0 ,  500 ,  600 ,  700 ,  8 5 0 ,  9 2 5

Atmospheric  u  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  v  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  t  5 0 ,  2 5 0 ,  5 00 ,  600 ,  700 ,  8 5 0 ,  9 2 5

Atmospheric  q  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  z  5 0 ,  2 5 0 ,  500 ,  600 ,  700 ,  8 5 0 ,  9 2 5

Atmospheric  u  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  v  50 ,  2 50 ,  500 ,  600 ,  700 ,  8 50 ,  9 2 5

Atmospheric  t  2 50 ,  500 ,  850

# D .   Q u a nt itat ive  eva l u at i o n

D . 1 .  M et ri cs

˜

This  section  presents  all  evaluation  metrics  we  use  in  Section  4 .  For  all  metrics ,  we  denote  ��  and  ��  as  the

prediction  and  ground  truth ,  which  have  a  shape  of  ��  ×   ��  ×   �� ,  where  ��  is  the  number  of  forecasts ,  or  the

number  of  test  samples ,  ��  ×   ��  is  the  spatial  resolution .  �� (�� )  is  the  latitude  weighting  term  to  account  for

33

ClimaX :  A  foundation  model  for  weather  and  climate

Ta ble  9 :  ECMWF  variables  used  in  our  ERA5  dataset .  Static  represents  variables  don ’t  depend  on  time ,

Single  represents  surface  variables ,  and  A tmospheric  represents  time-varying  atmospheric  properties  at  the

chosen  altitudes .

Type  Variable  name  Abbrev.  ECMWF  ID  Levels

Static  Land-sea  mask  LSM  1 72

Static  Orography

Single  2  metre  temperature  T2m  1 67

Single  1 0  metre  U  wind  component  U 1 0  1 65

Single  1 0  metre  V  wind  component  V 1 0  1 66

Atmospheric  Geopotential  Z  1 29  50 ,  2 50 ,  500 ,  600 ,  700 ,  850 ,  92 5

Atmospheric  U  wind  component  U  1 3 1  50 ,  2 50 ,  500 ,  600 ,  700 ,  850 ,  92 5

Atmospheric  V  wind  component  V  1 32  50 ,  2 50 ,  500 ,  600 ,  700 ,  850 ,  92 5

Atmospheric  Temperature  T  1 30  50 ,  2 50 ,  500 ,  600 ,  700 ,  850 ,  92 5

Atmospheric  Specific  humidity  Q  1 33  50 ,  2 50 ,  500 ,  600 ,  700 ,  850 ,  92 5

Atmospheric  Relative  humidity  R  1 5 7  50 ,  2 50 ,  500 ,  600 ,  700 ,  850 ,  92 5

the  non-uniformity  in  areas  of  the  grid  cells .  We  have  removed  the  time  notation  for  simplicity.

D . 1 . 1 .  Weather  forecasti ng  metrics

Root  mean  square  error  (RMSE)

��
 
 ��
 
 ��
 
 ˜

= 1 
 1 
 
 − 2

�� = 1  Ã �� = 1
 �� = 1

RMSE  
 ∑︁ ∑︁ ∑︁ �� (��) (���� ,��,��    ���� ,��,��  )
 .  (3)

��
 ��  ×   ��

Anomaly  correlation  coeffi˜cient  (AC C )  Anomaly  correlation  coefficient  (ACC)  is  the  spatial  correlation

′ 
 ′

between  prediction  anomalies  ��
 relative  to  climatology  and  ground  truth  anomalies  ��
 relative  to  climatology:

˜ ′ 
 ′

∑︀ �� , �� ,��  �� (�� ) ��
 �� , �� ,�� ��
�� , �� ,��

AC C  =
 
 ˜ ′ 
 ′ ,   ( 4 )

»∑︀ �� , �� ,��  �� (�� ) ��
 ��2, ��
 ,��
 ∑︀ �� , �� ,��  �� (�� ) ��
��2, ��
 ,��

˜ ′ 
 ˜ ′ 
 ′ 
 ′

��
 =  ��
 −  ��,  ��
 =  ��
 −  ��,  (5)

1

in  which  climatology  ��  is  the  temporal  mean  of  the  ground  truth  data  over  the  entire  test  set  ��  =
 ��
 ∑︀ ��  �� .

D . 1 . 2 .  C l i mate  projection  metrics

Normalized  spatial  root  mean  square  error  (NRMSE�� )  Normalized  spatial  root  mean  square  error

(NRMSE�� )  measures  the  spatial  discrepancy  between  the  temporal  mean  of  the  prediction  and  the  temporal

mean  of  the  ground  truth :

��
 
 ��
 
 2   ��

1 
 ˜ 1 
 1

��
 ∑︁ ��
 ∑︁ ¡��
 ∑︁

Õ

NRMSE��  =
 ��  −
 ��
 ⟨�� ⟩ ,  (6)

∞ (︃ ��= 1
 ��= 1
 )︃ ∫ ��= 1

in  which  ⟨��⟩   is  the  global  mean  of  �� :

��
 
 ��

1

⟨��⟩  =
 ∑︁ ∑︁ �� (��) ����,��  (7)

��  ×   ��

�� = 1 
 �� = 1

34

ClimaX :  A  foundation  model  for  weather  and  climate

Normalized  global  root  mean  square  error  (NRMSE�� )  Normalized  global  root  mean  square  error

(NRMSE�� )  measures  the  discrepancy  between  the  global  mean  of  the  prediction  and  the  global  mean  of  the

ground  trut h :

1 
 ��
 
 
 ˜ 2
   1 
 ��

Ã �� = 1
 �� = 1

��
 ¡ ��

NRMSE��  =
 ∑︁ Ä ⟨��⟩  −  ⟨��⟩
ä ∑︁ ⟨��⟩ .  (8)

Total  normalized  root  mean  square  error  (TRMSE)  Total  normalized  root  mean  square  error  (TRMSE)

is  the  weighted  sum  of  NRMSE��  and  NRMSE�� :

TRMSE  =  NRMSE��  +  ��  ·  NRMSE�� ,  (9)

where  ��  is  chosen  to  be  5  as  suggested  by  Watson-Parris ,  Rao ,  et  al .   [WP + 2 2] .

D . 1 . 3 .  C l i mate  downsca l i ng  metrics

Root  mean  square  error  (RMSE)  This  is  the  same  as  Equation  (3) .

Mean  bias  Mean  bias  measures  the  difference  between  the  spatial  mean  of  the  prediction  and  the  spatial

mean  of  the  ground  truth .  A  positive  mean  bias  shows  an  overestimation ,  while  a  negative  mean  bias  shows

an  underestimation  of  the  mean  value .

��
 
 ��
 
 ��
 
 ˜ ��
 
 ��
 
 ��

1 
 1

Mean  bias  =
 ��  −
 ��  ( 1 0)

��  ×  ��  ×  ��
 ∑︁ ∑︁ ∑︁ ��  ×  ��  ×  ��
 ∑︁ ∑︁ ∑︁

�� = 1
 �� = 1
 �� = 1
 �� = 1
 �� = 1
 �� = 1

Pearson  coefficient  Pearson  coefficient  measures  the  correlation  between  the  prediction  and  the  ground

truth .  We  first  flatten  the  prediction  and  ground  truth ,  and  compute  the  metric  as  follows :

˜

˜ cov (��  ,  ��  )

����  , �� =
 ˜ ( 1 1 )

���� ����

D . 2 .  Resu lts  su m mary

Table  1 0  and  1 1  summarize  the  global  forecasting  results  of  ClimaX  and  the  baselines  for  all  target  variables

and  at  all  lead  times .  In  addition  to  IFS  and  the  two  CNN-based  baselines  in  the  main  text ,  we  include

FourCastNet  [Pat+22] ,  PanguWeather  [Bi+22] ,  and  GraphCast  [Lam+22]  for  comprehensiveness .  We  want  to

emphasize  that  the  results  obtained  by  these  methods  are  not  comparable  with  ClimaX ,  as  they  were  trained

° ° °

on  ERA5  at  0 . 2 5 ,  a  much  higher  resolution  compared  to  5 . 62 5   and  1 . 4062 5   data  used  to  train  ClimaX .  In

Section  4 . 5 ,  we  had  a  discussion  on  how  the  performance  of  ClimaX  scales  favorably  with  respect  to  data

resolution .  We  hope  this  summary  will  provide  future  works  with  an  easier  comparison  with  existing  baselines .

In  spite  of  being  trained  on  much  lower  resolutions ,  ClimaX  outperforms  FourCastNet  in  forecasting  Z500 ,

T850 ,  and  U 1 0  at  lead  times  from  3  days  and  beyond ,  in  terms  of  both  RMSE  and  AC C .  For  T2m ,  ClimaX

achieves  better  results  at  horizons  longer  than  3  days .  PanguWeather  performs  better  than  ClimaX  on  most

of  the  tasks ,  but  the  gap  between  the  two  methods  shrinks  and  becomes  negligible  as  the  lead  time  increases .

ClimaX  even  outperforms  PanguWeather  in  predicting  U 1 0  at  7  days  lead  times .  This  is  because  ClimaX

is  finetuned  to  perform  direct  prediction ,  which  mitigates  error  accumulation  for  long  horizon  prediction .

GraphCast  achieves  the  lowest  RMSE  among  all  methods ,  but  performs  worse  in  terms  of  AC C  compared  to

ClimaX  and  PanguWeather.

35

ClimaX :  A  foundation  model  for  weather  and  climate

# Ta ble  1 0 :  RMSE  on  global  forecasting  for  different  target  variables  at  different  lead  times .  Lower  is  better .

# Va r  i a b  l e

L e a d  t  i m e   ClimaX   FCN a   PWb   GC c   HRES   IFS   ResNet   UNet

[hr . ]   5 . 6 2 5 °   1 . 40 6 2 5 °   0 . 2 5 °   0 . 2 5 °   0 . 2 5 °   0 . 1   5 . 6 2 5 °   1 . 40 6 2 5 °   5 . 6 2 5 °   5 . 6 2 5 °

Z 5 00   6   6 2 . 73   49 . 6 7   3 7 . 5 2   1 5 . 40   1 6 . 46   24 . 66   2 6 . 93   2 6 . 96   47 . 00   5 3 . 66

[m2 / s2 ]
   24   9 6 . 1 9   72 . 76   8 1 . 3 1   42 . 2 3   3 8 . 77   45 . 90   5 1 . 0 1   5 0 . 9 6   8 6 . 60   1 3 2 . 6 5

72   244 . 08   2 0 1 . 00   2 5 1 . 96   1 33 . 1 2   1 2 5 . 78   1 46 . 3 7   1 5 2 . 1 5   1 5 2 . 2 0   30 5 . 2 2   45 8 . 84

1 20   440 . 40   39 2 . 00   483 . 44   2 9 5 . 63   2 7 1 . 6 5   3 1 6 . 79   33 1 . 45   33 1 . 38   6 1 4 . 20   72 1 . 83

1 68   5 99 . 43   5 66 . 00   680 . 00   504 . 90   466 . 5 3   5 3 5 . 93   549 . 0 1   548 . 96   806 . 5 9   8 1 9 . 39

336   790 . 26   788 . 43   nan   nan   nan   nan   1 0 1 1 . 72   1 0 1 1 . 56   835 . 5 5   866 . 40

720   8 1 5 . 2 5   8 1 7 . 5 2   nan   nan   nan   nan   nan   nan   858 . 98   880 . 34

T 2 m   6   0 . 9 5   1 . 1 1   0 . 72   0 . 5 9   0 . 5 0   0 . 3 5   0 . 9 7   0 . 9 7   0 . 76   0 . 77

[K]   2 4   1 . 1 0   1 . 1 9   0 . 9 5   0 . 72   0 . 6 2   0 . 6 6   1 . 0 2   1 . 0 2   0 . 9 1   1 . 1 1

72   1 . 43   1 . 4 7   1 . 3 8   1 . 0 5   0 . 94   1 . 0 6   1 . 3 0   1 . 3 0   1 . 70   1 . 9 1

1 2 0   1 . 8 3   1 . 8 3   1 . 9 9   1 . 5 3   1 . 3 6   1 . 5 2   1 . 72   1 . 7 1   2 . 2 2   2 . 49

1 6 8   2 . 1 8   2 . 1 7   2 . 54   2 . 0 6   1 . 8 8   2 . 0 6   2 . 24   2 . 2 3   2 . 6 6   2 . 6 6

336   2 . 6 1   2 . 6 7   nan   nan   nan   nan   3 . 3 1   3 . 30   2 . 86   2 . 79

720   2 . 6 7   2 . 74   nan   nan   nan   nan   nan   nan   2 . 86   2 . 8 1

T8 5 0   6   0 . 8 8   0 . 84   0 . 5 2   0 . 42   0 . 2 8   0 . 3 3   0 . 6 9   0 . 6 9   0 . 70   0 . 80

[K]   2 4   1 . 1 1   1 . 0 2   0 . 8 1   0 . 72   0 . 5 8   0 . 70   0 . 8 7   0 . 8 7   1 . 2 6   1 . 2 5

72   1 . 5 9   1 . 46   1 . 5 5   1 . 1 3   1 . 0 2   1 . 2 7   1 . 34   1 . 34   1 . 9 0   2 . 3 9

1 2 0   2 . 2 3   2 . 0 8   2 . 4 7   1 . 78   1 . 6 3   1 . 9 6   2 . 0 1   2 . 0 1   2 . 8 6   3 . 2 3

1 6 8   2 . 77   2 . 6 6   3 . 3 0   2 . 6 0   2 . 4 1   2 . 78   2 . 8 2   2 . 8 2   3 . 5 1   3 . 5 0

336   3 . 40   3 . 4 1   nan   nan   nan   nan   4 . 43   4 . 43   3 . 6 5   3 . 6 5

720   3 . 47   3 . 49   nan   nan   nan   nan   nan   nan   3 . 69   3 . 73

U 1 0   6   1 . 0 8   1 . 04   0 . 5 5   0 . 46   0 . 3 7   0 . 5 8   0 . 8 0   0 . 79   0 . 8 6   1 . 0 2

[m / s]   2 4   1 . 4 1   1 . 3 1   0 . 9 9   0 . 9 0   0 . 8 0   1 . 1 5   1 . 1 1   1 . 1 1   1 . 2 7   1 . 6 8

72   2 . 1 8   2 . 0 2   2 . 2 4   1 . 6 0   1 . 4 7   1 . 9 8   1 . 9 2   1 . 9 2   2 . 78   3 . 1 7

1 2 0   2 . 94   2 . 79   3 . 4 1   2 . 5 2   2 . 3 6   2 . 9 5   2 . 8 9   2 . 8 9   3 . 6 3   3 . 9 3

1 6 8   3 . 43   3 . 3 5   4 . 1 8   3 . 46   3 . 2 5   3 . 8 7   3 . 8 1   3 . 8 1   4 . 1 5   4 . 0 8

336   3 . 9 1   3 . 9 2   nan   nan   nan   nan   5 . 24   5 . 2 3   4 . 2 3   4 . 1 6

720   3 . 96   3 . 9 7   nan   nan   nan   nan   nan   nan   4 . 29   4 . 2 2

a   FourCastNet  [Pat + 22]

b   PanguWeather  [Bi+22]

c   GraphCast  [Lam+22]

36

ClimaX :  A  foundation  model  for  weather  and  climate

# Ta ble  1 1 :  AC C  on  global  forecasting  for  different  target  variables  at  different  lead  times .  Higher  is  better .

# Va r  i a b  l e

L e a d  t  i m e   ClimaX   FCN a  PWb  GC c  HRES   IFS   ResNet   UNet

[hr . ]   5 . 6 2 5 °   1 . 40 6 2 5 °   0 . 2 5 °   0 . 2 5 °   0 . 2 5 °   0 . 1   5 . 6 2 5 °   1 . 40 6 2 5 °   5 . 6 2 5 °   5 . 6 2 5 °

Z 5 0 0   6   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0

2 4   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   1 . 0 0   0 . 9 9

72   0 . 9 7   0 . 9 8   0 . 9 7   0 . 9 9   0 . 9 9   0 . 9 8   0 . 9 9   0 . 9 9   0 . 9 5   0 . 8 9

1 2 0   0 . 9 0   0 . 9 2   0 . 8 9   0 . 9 6   0 . 94   0 . 9 2   0 . 9 5   0 . 9 5   0 . 79   0 . 6 9

1 6 8   0 . 8 0   0 . 8 2   0 . 76   0 . 8 7   0 . 8 3   0 . 78   0 . 8 7   0 . 8 7   0 . 5 7   0 . 5 7

336   0 . 5 9   0 . 5 9   nan   nan   nan   nan   0 . 5 5   0 . 5 5   0 . 5 3   0 . 5 1

720   0 . 5 5   0 . 5 5   nan   nan   nan   nan   nan   nan   0 . 49   0 . 49

T 2m   6   0 . 9 8   0 . 9 8   0 . 9 9   0 . 9 9   0 . 9 8   0 . 9 9   0 . 9 9   0 . 9 9   0 . 9 9   0 . 9 9

24   0 . 9 8   0 . 9 7   0 . 9 8   0 . 9 9   0 . 9 8   0 . 9 8   0 . 9 9   0 . 9 9   0 . 9 8   0 . 9 8

72   0 . 9 6   0 . 9 6   0 . 9 6   0 . 9 8   0 . 9 5   0 . 94   0 . 9 8   0 . 9 8   0 . 94   0 . 9 3

1 2 0   0 . 94   0 . 94   0 . 9 2   0 . 9 5   0 . 9 0   0 . 8 8   0 . 9 6   0 . 9 6   0 . 9 0   0 . 8 8

1 6 8   0 . 9 1   0 . 9 1   0 . 8 7   0 . 9 2   0 . 8 1   0 . 77   0 . 9 3   0 . 9 3   0 . 8 6   0 . 8 6

336   0 . 86   0 . 8 5   nan   nan   nan   nan   0 . 8 5   0 . 8 5   0 . 83   0 . 84

720   0 . 85   0 . 84   nan   nan   nan   nan   nan   nan   0 . 83   0 . 83

T8 5 0   6   0 . 9 8   0 . 9 9   0 . 9 9   1 . 00   1 . 00   0 . 9 9   0 . 9 9   0 . 9 9   0 . 9 9   0 . 9 9

24   0 . 9 8   0 . 9 8   0 . 9 8   0 . 9 9   0 . 9 9   0 . 9 8   0 . 9 9   0 . 9 9   0 . 9 7   0 . 9 7

72   0 . 9 5   0 . 9 6   0 . 9 5   0 . 9 8   0 . 9 6   0 . 9 3   0 . 9 7   0 . 9 7   0 . 9 2   0 . 8 8

1 2 0   0 . 8 9   0 . 9 1   0 . 8 7   0 . 94   0 . 8 9   0 . 84   0 . 9 3   0 . 94   0 . 8 2   0 . 75

1 6 8   0 . 8 2   0 . 84   0 . 77   0 . 8 7   0 . 75   0 . 6 8   0 . 8 7   0 . 8 7   0 . 6 8   0 . 6 9

336   0 . 7 1   0 . 7 1   nan   nan   nan   nan   0 . 68   0 . 69   0 . 66   0 . 66

720   0 . 69   0 . 68   nan   nan   nan   nan   nan   nan   0 . 64   0 . 64

U 1 0   6   0 . 9 7   0 . 9 7   0 . 9 9   0 . 9 9   0 . 9 9   0 . 9 9   0 . 9 8   0 . 9 8   0 . 9 8   0 . 9 7

24   0 . 94   0 . 9 5   0 . 9 7   0 . 9 7   0 . 9 8   0 . 9 6   0 . 9 7   0 . 9 7   0 . 9 5   0 . 9 1

72   0 . 8 5   0 . 8 7   0 . 8 5   0 . 9 2   0 . 9 3   0 . 8 8   0 . 8 9   0 . 8 9   0 . 74   0 . 6 5

1 2 0   0 . 70   0 . 74   0 . 64   0 . 80   0 . 8 2   0 . 74   0 . 76   0 . 76   0 . 5 2   0 . 3 7

1 6 8   0 . 5 6   0 . 5 9   0 . 45   0 . 6 3   0 . 64   0 . 5 5   0 . 5 8   0 . 5 8   0 . 2 8   0 . 2 8

336   0 . 33   0 . 3 2   nan   nan   nan   nan   0 . 2 1   0 . 2 1   0 . 1 9   0 . 2 2

720   0 . 29   0 . 28   nan   nan   nan   nan   nan   nan   0 . 1 7   0 . 2 1

a  FourCastNet  [Pat + 22]

b   PanguWeather  [Bi+22]

c  GraphCast  [Lam+22]

37

##### ClimaX :  A  foundation  model  for  weather  and  climate

###### E .   Q u a l ita t i ve  eva l u a t i o n

#### We  qualitatively  evaluate  the  performance  of  CliMax  on  global  forecasting  tasks  for  all  target  variables  and  at

#### all  lead  times .  In  each  figure ,  the  first  column  is  the  initial  condition  of  the  t arget  variable ,  which  serves  as  the

#### input ,  the  second  column  is  the  ground  truth  of  the  target  variable  at  a  particular  lead  time ,  the  third  column

#### is  the  prediction  of  ClimaX ,  and  the  last  column  is  the  bias ,  which  is  the  difference  between  the  prediction

#### and  the  ground  truth .

###### E . 1 .  N owcasti ng

### I n i t i a l  c o n d i t i o n   5 8000 
 G ro u n d  t ru t h   5 8000 
 6 h rs  P re d i c t i o n   5 8000 
 B i a s

2 0 0

##### 5 6 0 0 0 
 5 6 0 0 0 
 5 6 0 0 0

1 0 0

##### 0 
 5 4 0 0 0 
 5 4 0 0 0 
 5 4 0 0 0

###### 05 5 2 000 
 5 2 000 
 5 2 000 
 0

##### Z 5 0 0 0 0 
 5 0 0 0 0 
 5 0 0 0 0 
 1 0 0

##### 4 8 0 0 0 
 4 8 0 0 0 
 4 8 0 0 0 
 2 0 0

## I n i t i a l c o n d i t i o n   G ro u n d t ru t h   6 h rs P re d i ct i o n   B i a s

3 0 0

3 0 0 
 3 0 0 
 3 0 0 
 1 0

##### m 2 8 0 
 2 8 0 
 2 8 0 
 5

##### 2 2 6 0 
 2 6 0 
 2 6 0 
 0

# T

2 4 0 
 2 4 0 
 2 4 0 
 5

2 2 0 
 2 2 0

### I n i t i a l  c o n d i t i o n   G ro u n d t ru t h   300 
 6 h rs  P re d i ct i o n   300 
 B i a s

3 0 0 
 6

2 9 0 
 2 9 0 
 2 9 0 
 4

0 
 2 8 0 
 2 8 0 
 2 8 0 
 2

###### 85 2 7 0 
 2 7 0 
 2 7 0 
 0

##### T 2 6 0 
 2 6 0 
 2 6 0 
 2

2 5 0 
 2 5 0 
 2 5 0 
 46

2 4 0 
 2 4 0 
 2 4 0 
 8

## I n i t i a l c o n d i t i o n   G ro u n d t ru t h   6 h rs  P re d i ct i o n   B i a s 
 10

1 5 
 1 5 
 1 5

1 0 
 1 0 
 1 0 
 5

0 
 5 
 5 
 5

1 0 
 0 
 0 
 0

U 5 
 5 
 5

1 0 
 1 0 
 1 0 
 5

1 5 
 1 5 
 1 5

2 0 
 1 0

#### Figu re  14 :  Example  forecasts  from  ClimaX  at  6-hour  lead  time  compared  to  ground  truth  ERA5 .

##### 38

##### ClimaX :  A  foundation  model  for  weather  and  climate

###### E . 2 .  S hort  a nd  med i u m- ra nge  weather  forecasti ng

### I n i t i a l  c o n d i t i o n   5 8000 
 G ro u n d  t ru t h   5 8000 
 1 d a y  P re d i c t i o n   5 8000 
 B i a s 
 600

##### 5 6 0 0 0 
 5 6 0 0 0 
 5 6 0 0 0 
 4 0 0

##### 0 
 5 4 0 0 0 
 5 4 0 0 0 
 5 4 0 0 0 
 2 0 0

#### 05 5 2 000 
 5 2 000 
 5 2 000 
 0

###### Z 5 0 0 0 0 
 5 0 0 0 0 
 5 0 0 0 0 
 2 0 0

##### 4 8 0 0 0 
 4 8 0 0 0 
 4 8 0 0 0 
 4 0 0

## I n i t i a l c o n d i t i o n   G ro u n d t ru t h   1 d a y P re d i ct i o n   B i a s

3 0 0 
 3 0 0 
 3 0 0 
 1 0

2 8 0 
 2 8 0 
 2 8 0 
 5

###### m

##### 2 2 6 0 
 2 6 0 
 2 6 0 
 0

# T

2 4 0 
 2 4 0 
 2 4 0 
 5

2 2 0

###### I n i t i a l  c o n d i t i o n   G ro u n d t ru t h   300 
 1 d a y  P re d i ct i o n   300 
 B i a s

3 0 0 
 2 9 0 
 2 9 0 
 8

2 9 0 
 6

0 
 2 8 0 
 2 8 0 
 2 8 0 
 4

##### 5 2 7 0 
 2 7 0 
 2 7 0 
 2

#### 8T 2 60 
 2 60 
 2 60 
 0

2 5 0 
 2 5 0 
 2 5 0 
 42

2 4 0 
 2 4 0 
 2 4 0 
 6

###### I n i t i a l  c o n d i t i o n   G ro u n d t ru t h   2 0 
 1 d a y  P re d i ct i o n   2 0 
 B i a s

1 5 
 1 5 
 1 5 
 1 0

51 0 
 
 1 0 
 1 0 
 5

01 
 0 
 5 
 5 
 0

U 5 
 0 
 0 
 5

1 0 
 5 
 5 
 1 0

1 5 
 1 0 
 1 0

2 0 
 1 5 
 1 5 
 1 5

###### Figu re  1 5 :  Example  forecasts  from  ClimaX  at  1-day  lead  time  compared  to  ground  truth  ERA5 .

### I n i t i a l  c o n d i t i o n   58000 
 G ro u n d  t ru t h   58000 
 3 - d a y  P re d i ct i o n   58000 
 B i a s

##### 5 6 0 0 0 
 5 6 0 0 0 
 5 6 0 0 0 
 2 0 0 0

##### 0 
 5 4 0 0 0 
 5 4 0 0 0 
 5 4 0 0 0 
 1 0 0 0

#### 05 5 2 000 
 5 2 000 
 5 2 000 
 0

#### Z 5 0 0 0 0 
 5 0 0 0 0 
 5 0 0 0 0 
 1 0 0 0

##### 4 8 0 0 0 
 4 8 0 0 0 
 4 8 0 0 0 
 2 0 0 0

## I n i t i a l c o n d i t i o n   G ro u n d t ru t h   3 - d a y P re d i ct i o n   B i a s

3 0 0 
 3 0 0 
 3 0 0 
 1 0

2 8 0 
 2 8 0 
 2 8 0 
 5

##### m 
 0

##### 2 2 6 0 
 2 6 0 
 2 6 0 
 5

# T

2 4 0 
 2 4 0 
 2 4 0 
 1 0

2 2 0 
 2 2 0 
 1 5

###### I n i t i a l  c o n d i t i o n   G ro u n d t ru t h   3 - d a y  P re d i ct i o n   300 
 B i a s 
 10

3 0 0 
 3 0 0

2 9 0 
 2 9 0 
 2 9 0 
 5

0 
 2 8 0 
 2 8 0 
 2 8 0 
 0

#### 85 2 7 0 
 2 7 0 
 2 7 0

##### T 2 6 0 
 2 6 0 
 2 6 0 
 5

2 5 0 
 2 5 0 
 2 5 0 
 1 0

2 4 0 
 2 4 0 
 2 4 0

## I n i t i a l c o n d i t i o n   G ro u n d t ru t h   20 
 3 - d a y  P re d i ct i o n   B i a s

1 5 
 1 5 
 1 5 
 2 0

51 0 
 
 1 0 
 1 0 
 1 0

0 
 0 
 5 
 5

##### 1U 5 
 0 
 0 
 0

1 0 
 5 
 5

1 5 
 1 0 
 1 0 
 1 0

2 0 
 1 5

###### Figu re  1 6 :  Example  forecasts  from  ClimaX  at  3-day  lead  time  compared  to  ground  truth  ERA5 .

###### 39

#### ClimaX :  A  foundation  model  for  weather  and  climate

## I n i t i a l  c o n d i t i o n   5 8000 
 G ro u n d  t ru t h   5 8000 
 5 - d a y  P re d i c t i o n   5 8000 
 B i a s 
 4000

#### 5 6 0 0 0 
 5 6 0 0 0 
 5 6 0 0 0 
 3 0 0 0

#### 0 
 5 4 0 0 0 
 5 4 0 0 0 
 5 4 0 0 0 
 2 0 0 0

### 05 52000 
 1000

###### Z 5 2 0 0 0 
 5 2 0 0 0 
 0

#### 5 0 0 0 0 
 5 0 0 0 0 
 1 0 0 0

#### 4 8 0 0 0 
 5 0 0 0 0 
 2 0 0 0

#### 4 8 0 0 0

###### I n i t i a l c o n d i t i o n   G ro u n d t ru t h   5 - d a y  P re d i ct i o n   B i a s 
 20

3 0 0 
 3 0 0 
 3 0 0

2 8 0 
 2 8 0 
 2 8 0 
 1 0

#### 2m 
 2 6 0 
 2 6 0 
 0

### T 260

2 4 0 
 2 4 0 
 1 0

2 4 0

2 2 0 
 2 2 0 
 2 0

###### I n i t i a l  c o n d i t i o n   G ro u n d  t ru t h   300 
 5 - d a y  P re d i ct i o n   300 
 B i a s 
 2 0

3 0 0 
 2 9 0 
 2 9 0 
 1 5

0 
 22 98 00 

 2 8 0 
 2 8 0 
 1 0

#### 5 2 7 0 
 2 7 0 
 2 7 0 
 5

###### 8T 2 60 
 2 60 
 2 60 
 0

2 5 0 
 2 5 0 
 2 5 0 
 5

2 4 0 
 2 4 0 
 2 4 0 
 1 0

###### I n i t i a l c o n d i t i o n   G ro u n d t ru t h   5 - d a y  P re d i ct i o n   B i a s 
 20

1 5 
 1 5 
 1 0

1 0 
 1 0 
 1 0

0 
 5 
 5 
 5

1 0 
 0 
 0 
 0

#### U 5 
 5

1 0 
 1 0 
 5 
 1 0

1 5 
 1 5 
 1 0

2 0 
 2 0

### Figu re  1 7 :  Example  forecasts  from  ClimaX  at  5-day  lead  time  compared  to  ground  truth  ERA5 .

# I n i t i a l  c o n d i t i o n   58000 
 G ro u n d t ru t h   7 - d a y  P re d i ct i o n   B i a s 
 3000

#### 5 6 0 0 0 
 5 6 0 0 0 
 5 6 0 0 0 
 2 0 0 0

#### 0 
 5 4 0 0 0 
 5 4 0 0 0 
 5 4 0 0 0 
 1 0 0 0

###### 05 5 2 000 
 5 2 000 
 5 2 000 
 0

###### Z 5 0 0 0 0 
 5 0 0 0 0 
 1 0 0 0

#### 4 8 0 0 0 
 4 8 0 0 0 
 5 0 0 0 0 
 2 0 0 0

###### I n i t i a l c o n d i t i o n   G ro u n d t ru t h   7 - d a y P re d i ct i o n   B i a s

3 0 0 
 3 0 0 
 3 0 0 
 2 0

2 9 0

2 8 0 
 2 8 0 
 2 8 0 
 1 0

#### m 
 2 7 0

#### 2 2 6 0 
 2 6 0 
 2 6 0 
 0

#### T 2 4 0 
 2 4 0 
 2 5 0 
 1 0

2 4 0

2 2 0 
 2 3 0 
 2 0

###### I n i t i a l c o n d i t i o n   G ro u n d t ru t h   300 
 7 - d a y  P re d i ct i o n   B i a s

3 0 0 
 2 9 0 
 2 9 0 
 1 5

#### 0 2 9 0 
 2 8 0 
 2 8 0 
 1 0

2 8 0 
 5

###### 85 2 7 0 
 2 7 0 
 2 7 0 
 0

#### T 2 6 0 
 2 6 0 
 2 6 0 
 5

2 5 0 
 2 5 0 
 2 5 0 
 1 0

2 4 0 
 2 4 0 
 1 5

# I n i t i a l  c o n d i t i o n   G ro u n d t ru t h   2 0 
 7 - d a y  P re d i ct i o n   B i a s 
 1 5

1 5 
 1 0

1 0 
 1 0 
 5 
 51 0

0 
 05 

 0 
 0

1 5 
 0 
 5

U 1 0 
 1 0 
 5 
 1 0

1 5 
 1 0 
 1 5

2 0 
 2 0

### Figu re  1 8 :  Example  forecasts  from  ClimaX  at  7-day  lead  time  compared  to  ground  truth  ERA5 .

#### 40

### ClimaX :  A  foundation  model  for  weather  and  climate

## E . 3 .  Longer  horizon  i nsta nta neous  forecasti ng

# I n i t i a l c o n d i t i o n   58000 
 G ro u n d t ru t h   2 w e e ks  P re d i ct i o n   B i a s 
 3000

### 5 6 0 0 0 
 5 6 0 0 0 
 5 6 0 0 0 
 2 0 0 0

### 0 
 5 4 0 0 0 
 5 4 0 0 0 
 5 4 0 0 0 
 1 0 0 0

## 05 5 2 000 
 5 2 000 
 0

###### Z 5 0 0 0 0 
 5 0 0 0 0 
 5 2 0 0 0 
 1 0 0 0

### 4 8 0 0 0 
 4 8 0 0 0 
 5 0 0 0 0 
 2 0 0 0

###### I n i t i a l c o n d i t i o n   G ro u n d t ru t h   2 we e ks P re d i ct i o n   B i a s

### 3 0 0 0

3 0 0 
 3 0 0 
 3 0 0 
 1 0

2 9 0

2 8 0 
 2 8 0 
 2 8 0 
 0

###### 2m 
 2 6 0 
 2 6 0 
 2 7 0

###### T 2 60 
 1 0

2 4 0 
 2 4 0 
 2 5 0 
 2 0

2 4 0

2 2 0

# I n i t i a l c o n d i t i o n   G ro u n d t ru t h   300 
 2 we e ks P re d i ct i o n   B i a s

3 0 0 
 2 9 0 
 2 9 0 
 1 0

2 9 0

0 
 2 8 0 
 2 8 0 
 2 8 0 
 0

## 85 2 7 0 
 2 7 0 
 2 7 0

### T 2 6 0 
 2 6 0 
 2 6 0 
 1 0

2 5 0 
 2 5 0 
 2 5 0 
 2 0

2 4 0

# I n i t i a l c o n d i t i o n   G ro u n d t ru t h   2 0 
 2 w e e ks  P re d i ct i o n   10 
 B i a s

1 5 
 2 0

1 0 
 1 0 
 5

0 
 5 
 1 0

1 0 
 0 
 0

### U 5 
 0

1 0 
 1 0 
 5 
 1 0

1 5

2 0 
 2 0 
 1 0

## Figu re  1 9 :  Example  forecasts  from  ClimaX  at  2-week  lead  time  compared  to  ground  truth  ERA5 .

# I n i t i a l c o n d i t i o n   58000 
 G ro u n d t ru t h   58000 
 1 m o n t h  P re d i ct i o n   B i a s

### 3 0 0 0

### 5 6 0 0 0 
 5 6 0 0 0 
 5 6 0 0 0 
 2 0 0 0

### 0 
 5 4 0 0 0 
 5 4 0 0 0 
 5 4 0 0 0 
 1 0 0 0

## 05 5 2 000 
 5 2 000 
 0

###### Z 5 0 0 0 0 
 5 0 0 0 0 
 5 2 0 0 0 
 1 0 0 0

### 4 8 0 0 0 
 4 8 0 0 0 
 5 0 0 0 0 
 23 00 00 00

###### I n i t i a l c o n d i t i o n   G ro u n d t ru t h   1 m o n t h P re d i ct i o n   B i a s 
 20

3 0 0 
 3 0 0 
 3 0 0

2 9 0 
 1 0

2 8 0 
 2 8 0 
 2 8 0

### 2m 
 2 6 0 
 2 6 0 
 2 7 0 
 0

###### T 2 60 
 1 0

2 4 0 
 2 4 0 
 2 5 0

2 4 0 
 2 0

2 2 0

# I n i t i a l c o n d i t i o n   G ro u n d t ru t h   300 
 1 m o n t h  P re d i ct i o n   B i a s 
 1 5

3 0 0 
 2 9 0 
 2 9 0 
 1 0

0 
 22 98 00 

 2 8 0 
 2 8 0 
 5

###### 85 2 7 0 
 2 7 0 
 2 7 0 
 0

### T 2 6 0 
 2 6 0 
 5

2 5 0 
 2 5 0 
 2 6 0 
 1 0

2 4 0 
 2 4 0 
 2 5 0 
 1 5

# I n i t i a l c o n d i t i o n   G ro u n d t ru t h   2 0 
 1 m o n t h  P re d i ct i o n   B i a s 
 2 0

1 5 
 7 . 5 
 1 5

1 0 
 1 0 
 5 . 0 
 1 0

0 
 5 
 2 . 5 
 5

1 0 
 0 
 0 . 0 
 0

### U 5 
 2 . 5 
 5

1 0 
 1 0 
 5 . 0 
 1 0

1 5 
 7 . 5 
 1 5

2 0 
 2 0 
 1 0 . 0

## Figu re  20 :  Example  forecasts  from  ClimaX  at  1-month  lead  time  compared  to  ground  truth  ERA5 .

###### 4 1
