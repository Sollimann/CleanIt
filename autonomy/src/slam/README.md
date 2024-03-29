## Mobile robot localization

In **odometry** (wheel sensors only) and **dead reckoning** (also heading sensors) the position update is based on proprioceptive sensors.
The movement of the robot, sensed with wheel encoders or heading sensors or both, is integrated to compute position. Because the
sensor measurement errors are integrated, the position error accumulates over time. Thus the position has to be updated
from time to time by other localization mechanisms. Otherwise the robot is not able to maintain a meaningful position estimate in the long run. 

The algorithm we are implementing for odometry assumes we have wheel sensor readings of a differential-drive robot only.
Using additional heading sensors (i.e gyroscope) can help to reduce the cumulative errors.

There are many sources of odometric error, some of them are:
* Limited resolution during integration (time increments, measurement resolution, etc.);
* Misalignment of the wheels (deterministic);
* Uncertainty in the wheel diameter and in particular unequal wheel diameter (deterministic);
* Variation in the contact point of the wheel;
* Unequal floor contact (slipping, nonplanar surface, etc.).

Some of the errors might be *deterministic (systematic)*, thus they can be eliminated by proper calibration of the system.
However, there are still a number of *nondeterministic (random)* errors which remain, leading to uncertainties in position
estimation over time. From a geometric point of view one can classify the errors into *three* types:

1. **Range error**: integrated path length (distance) of the robot's movement -> sum of the wheel movements
2. **turn error**: similar to range error, but for turns -> difference of the wheel motions
3. **Drift error**: difference in the error of the wheels leads to an error in the robot's angular orientation

Over long periods of time, turn and drift errors far outweigh range errors, since their contribution of the overall position
error is *nonlinear*. Consider a robot whose position is initially perfectly well-known, moving forward in a straight line
along the x-acis. The error in the *y*-position introduced by a move of of *d* meters will have a component of *d sin Δθ*,
which can be quite large as the angular error *Δθ* grows. Over time, as a mobile robot moves about the environment, the rotational
error between its internal reference frame and its original reference frame grows quickly. As the robot moves away from
the origin of these refrence frames, the resulting linear error in position grows quite large. It is instructive to establish
an error model for odometric accuracy and see how the errors propagate over time.

## *An error model for odometric position estimation*

![Coordinate frames](../../docs/coordinate_frame.png)

### **Step.1 : Calculate the pose**
Generally the pose (position) of a robot is represented by the vector: 

<img src="https://latex.codecogs.com/gif.latex?\bg_white&space;p=\left[\begin{array}{l}&space;x&space;\\&space;y&space;\\&space;\theta&space;\end{array}\right]" title="p=\left[\begin{array}{l} x \\ y \\ \theta \end{array}\right]" />

For a differential-drive robot the position can be estimated starting from a known position by integrating the movement
(summing the incremental travel distances). For a discrete system with a fixed sampling interval *Δt* the incremental travel
distances (*Δx*, *Δy*, *Δθ*) are:

<img src="https://latex.codecogs.com/gif.latex?\bg_white&space;\begin{array}{l}&space;\Delta&space;x=\Delta&space;s&space;\cos&space;(\theta&plus;\Delta&space;\theta&space;/&space;2)&space;\\&space;\Delta&space;y=\Delta&space;s&space;\sin&space;(\theta&plus;\Delta&space;\theta&space;/&space;2)&space;\\&space;\Delta&space;\theta=\frac{\Delta&space;s_{r}-\Delta&space;s_{l}}{b}&space;\\&space;\Delta&space;s=\frac{\Delta&space;s_{r}&plus;\Delta&space;s_{l}}{2}&space;\end{array}" title="\begin{array}{l} \Delta x=\Delta s \cos (\theta+\Delta \theta / 2) \\ \Delta y=\Delta s \sin (\theta+\Delta \theta / 2) \\ \Delta \theta=\frac{\Delta s_{r}-\Delta s_{l}}{b} \\ \Delta s=\frac{\Delta s_{r}+\Delta s_{l}}{2} \end{array}" />

where:

(*Δx*; *Δy*; *Δθ*) = path traveled in the last sampling interval.
(*Δs_r*; *Δs_l*) = traveled distances for the right and left wheel respectively.
*b* = distance between the two wheels of differential-drive robot

thus we get the updated position *p'*:

<img src="https://latex.codecogs.com/gif.latex?\bg_white&space;p^{\prime}=\left[\begin{array}{l}&space;x^{\prime}&space;\\&space;y^{\prime}&space;\\&space;\theta^{\prime}&space;\end{array}\right]=p&plus;\left[\begin{array}{c}&space;\Delta&space;s&space;\cos&space;(\theta&plus;\Delta&space;\theta&space;/&space;2)&space;\\&space;\Delta&space;s&space;\sin&space;(\theta&plus;\Delta&space;\theta&space;/&space;2)&space;\\&space;\Delta&space;\theta&space;\end{array}\right]=\left[\begin{array}{l}&space;x&space;\\&space;y&space;\\&space;\theta&space;\end{array}\right]&plus;\left[\begin{array}{c}&space;\Delta&space;s&space;\cos&space;(\theta&plus;\Delta&space;\theta&space;/&space;2)&space;\\&space;\Delta&space;s&space;\sin&space;(\theta&plus;\Delta&space;\theta&space;/&space;2)&space;\\&space;\Delta&space;\theta&space;\end{array}\right]" title="p^{\prime}=\left[\begin{array}{l} x^{\prime} \\ y^{\prime} \\ \theta^{\prime} \end{array}\right]=p+\left[\begin{array}{c} \Delta s \cos (\theta+\Delta \theta / 2) \\ \Delta s \sin (\theta+\Delta \theta / 2) \\ \Delta \theta \end{array}\right]=\left[\begin{array}{l} x \\ y \\ \theta \end{array}\right]+\left[\begin{array}{c} \Delta s \cos (\theta+\Delta \theta / 2) \\ \Delta s \sin (\theta+\Delta \theta / 2) \\ \Delta \theta \end{array}\right]" />

### **Step.2 : Calculate the error model**

The odometric position updates can give only a very rough estimate of the actual position. Owing to integration errors of the
uncertainties of *p* and the motion errors during the incremental motion (*Δs_l*; *Δs_r*) the position error based on odometry
grows with time

We have to establish an error model for the integrated position *p'* to obtain a *covariance matrix, Σ_p*, od the odometric
position estimate. To do so, we assume that the starting point of the initial covariance matrix *Σ_p* is known. For the motion
increment (*Δs_r*; *Δs_l*) we assume the following *covariance matrix, Σ_Δ*:

<img src="https://latex.codecogs.com/gif.latex?\Sigma_{\Delta}=\operatorname{covar}\left(\Delta&space;s_{r},&space;\Delta&space;s_{l}\right)=\left[\begin{array}{cc}&space;k_{r}\left|\Delta&space;s_{r}\right|&space;&&space;0&space;\\&space;0&space;&&space;k_{l}&space;\mid&space;\Delta&space;s_{l}&space;\end{array}\right]" title="\Sigma_{\Delta}=\operatorname{covar}\left(\Delta s_{r}, \Delta s_{l}\right)=\left[\begin{array}{cc} k_{r}\left|\Delta s_{r}\right| & 0 \\ 0 & k_{l} \mid \Delta s_{l} \end{array}\right]" />

where (*Δs_r*; *Δs_l*) are distances traveled by each wheel, and *k_r* and *k_l* are error contants representing the *nondeterministic*
parameters of the motor drive and the wheel-floor interaction. For this covariance estimate, the following assumptions have been made:

* The two errors of the individually driven wheels are independent.
* The variance of the errors (left and right wheels) are proportional to the absolute value of the traveled distances (*Δs_l*; *Δs_r*).

These assumptions, while not perfect, are suitable and will thus be used for the further development of the error model.
The *motion errors / velocity errors* are due to imprecise movement because of the deformation of wheel, slippage, unequal floor, errors in encoders,
and so on. The values for the errors konstant *k_r* and *k_l* depend on the robot and the environment and should be exprerimentally established
by performing and analyzing representative movements.

Assuming that *p* and *Δ_rl* = (*Δs_r*; *Δs_l*) are uncorrelated and the derivation of *p' = f(x,y,θ, Δs_r, Δs_l)* is reasonably
approximated by the first-order Taylor expansion (linearization) of *f*, it can be concluded, using the *error propagation law*:

<img src="https://latex.codecogs.com/gif.latex?C_{Y}=F_{X}&space;C_{X}&space;F_{X}^{T}" title="C_{Y}=F_{X} C_{X} F_{X}^{T}" />

where:
* *C_x* = covariance matrix representing the input uncertainties
* *C_y* = covariance matrix representing the propagated uncertainties for the outputs
* *F_x* = is the *Jacobian* matrix of the vehicle pose model *f*

Giving us the pose covariance estimate:

<img src="https://latex.codecogs.com/gif.latex?\Sigma_{p^{\prime}}=\nabla_{p}&space;f&space;\cdot&space;\Sigma_{p}&space;\cdot&space;\nabla_{p}&space;f^{T}&plus;\nabla_{\Delta_{r&space;l}}&space;f&space;\cdot&space;\Sigma_{\Delta}&space;\cdot&space;\nabla_{\Delta_{r&space;l}}&space;f^{T}" title="\Sigma_{p^{\prime}}=\nabla_{p} f \cdot \Sigma_{p} \cdot \nabla_{p} f^{T}+\nabla_{\Delta_{r l}} f \cdot \Sigma_{\Delta} \cdot \nabla_{\Delta_{r l}} f^{T}" />

The covariance matrix *Σ_p* is, of course, always given by the *Σ_p'* of the previous step and can thus be calculated after
specifying an initial value (e.g, 0)

where:

<img src="https://latex.codecogs.com/gif.latex?F_{p}=\nabla_{p}&space;f=\nabla_{p}\left(f^{T}\right)=\left[\frac{\partial&space;f}{\partial&space;x}&space;\frac{\partial&space;f}{\partial&space;y}&space;\frac{\partial&space;f}{\partial&space;\theta}\right]=\left[\begin{array}{ccc}&space;1&space;&&space;0&space;&&space;-\Delta&space;s&space;\sin&space;(\theta&plus;\Delta&space;\theta&space;/&space;2)&space;\\&space;0&space;&&space;1&space;&&space;\Delta&space;s&space;\cos&space;(\theta&plus;\Delta&space;\theta&space;/&space;2)&space;\\&space;0&space;&&space;0&space;&&space;1&space;\end{array}\right]" title="F_{p}=\nabla_{p} f=\nabla_{p}\left(f^{T}\right)=\left[\frac{\partial f}{\partial x} \frac{\partial f}{\partial y} \frac{\partial f}{\partial \theta}\right]=\left[\begin{array}{ccc} 1 & 0 & -\Delta s \sin (\theta+\Delta \theta / 2) \\ 0 & 1 & \Delta s \cos (\theta+\Delta \theta / 2) \\ 0 & 0 & 1 \end{array}\right]" />

<img src="https://latex.codecogs.com/gif.latex?F_{\Delta_{r&space;l}}=&space;\begin{bmatrix}&space;\frac{1}{2}&space;\cos&space;\left(\theta&plus;\frac{\Delta&space;\theta}{2}\right)-\frac{\Delta&space;s}{2&space;b}&space;\sin&space;\left(\theta&plus;\frac{\Delta&space;\theta}{2}\right)&space;&&space;\frac{1}{2}&space;\cos&space;\left(\theta&plus;\frac{\Delta&space;\theta}{2}\right)&plus;\frac{\Delta&space;s}{2&space;b}&space;\sin&space;\left(\theta&plus;\frac{\Delta&space;\theta}{2}\right)\\&space;\frac{1}{2}&space;\sin&space;\left(\theta&plus;\frac{\Delta&space;\theta}{2}\right)&plus;\frac{\Delta&space;s}{2&space;b}&space;\cos&space;\left(\theta&plus;\frac{\Delta&space;\theta}{2}\right)&space;&&space;\frac{1}{2}&space;\sin&space;\left(\theta&plus;\frac{\Delta&space;\theta}{2}\right)-\frac{\Delta&space;s}{2&space;b}&space;\cos&space;\left(\theta&plus;\frac{\Delta&space;\theta}{2}\right)&space;\\&space;\frac{1}{b}&space;&&space;-&space;\frac{1}{b}&space;\end{bmatrix}" title="F_{\Delta_{r l}}= \begin{bmatrix} \frac{1}{2} \cos \left(\theta+\frac{\Delta \theta}{2}\right)-\frac{\Delta s}{2 b} \sin \left(\theta+\frac{\Delta \theta}{2}\right) & \frac{1}{2} \cos \left(\theta+\frac{\Delta \theta}{2}\right)+\frac{\Delta s}{2 b} \sin \left(\theta+\frac{\Delta \theta}{2}\right)\\ \frac{1}{2} \sin \left(\theta+\frac{\Delta \theta}{2}\right)+\frac{\Delta s}{2 b} \cos \left(\theta+\frac{\Delta \theta}{2}\right) & \frac{1}{2} \sin \left(\theta+\frac{\Delta \theta}{2}\right)-\frac{\Delta s}{2 b} \cos \left(\theta+\frac{\Delta \theta}{2}\right) \\ \frac{1}{b} & - \frac{1}{b} \end{bmatrix}" />

and with:

<img src="https://latex.codecogs.com/gif.latex?\Delta&space;s=\frac{\Delta&space;s_{r}&plus;\Delta&space;s_{l}}{2}&space;;&space;\quad&space;\Delta&space;\theta=\frac{\Delta&space;s_{r}-\Delta&space;s_{l}}{b}" title="\Delta s=\frac{\Delta s_{r}+\Delta s_{l}}{2} ; \quad \Delta \theta=\frac{\Delta s_{r}-\Delta s_{l}}{b}" />

