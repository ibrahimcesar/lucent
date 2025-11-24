// Dart example file

/// A simple calculator class
class Calculator {
  /// Add two numbers
  int add(int a, int b) {
    return a + b;
  }

  /// Multiply two numbers
  int multiply(int a, int b) => a * b;

  /// Check if number is even
  bool isEven(int n) {
    return n % 2 == 0;
  }

  // Private method
  void _validate(int n) {
    if (n < 0) throw ArgumentError('Negative number');
  }
}

/// Main function
void main() {
  final calc = Calculator();
  print(calc.add(5, 3));
}
