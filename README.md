# Kepler Equation Solver

Kepler's equation relates the eccentricity, mean anomaly, and eccentric anomaly of a planet
in its orbit around the Sun. 
- The eccentricity of an orbit represents how far the orbit deviates from
a perfect circle.

- The mean anomaly is a time-dependent variable that gives the angle of the planet from
its point of perihelion, as it would be if the planet were moving on a perfect circle.

- The eccentric anomaly is the actual angle of the planet
on its elliptical orbit, from the point of perihelion, measured from the center of the foci.
In other words, the eccentric anomaly is very close to the true anomaly, measured at the Sun
or central body.

## Kepler's Equation

$M$ is the mean anomaly.

$e$ is the orbital eccentricity.

$E$ is the eccentric anomaly.

$$M = E - e\sin (E)$$

This is a trancendental equation, and no closed-form solution exists for $E$. Therefore, we use
the Newton-Raphson iteration on the modified form

$$f(E) = 0 = E - e\sin (E) - M$$

and its derivative

$$f'(E) = 1 - e\cos (E).$$

Newton-Raphson iteration lets us find solutions to things like
$$\Phi (x) = 0$$
or in other words, it's a root-finding algorithm. It works by
choosing an initial guess for the solution, and then doing
$$x_{n + 1} = x_n - \frac{\Phi (x)}{\Phi '(x)}$$
and letting $x_n$ converge to the result.
