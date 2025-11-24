// Scala example file
package calculator

/** Calculator object with basic operations */
object Calculator {

  // Function to add two numbers
  def add(a: Int, b: Int): Int = {
    a + b
  }

  // Function to multiply
  def multiply(a: Int, b: Int): Int = a * b

  /**
   * Calculate factorial recursively
   * @param n the number
   * @return factorial of n
   */
  def factorial(n: Int): Int = {
    if (n <= 1) 1
    else n * factorial(n - 1)
  }

  // Private helper function
  private def validate(n: Int): Boolean = n >= 0
}
