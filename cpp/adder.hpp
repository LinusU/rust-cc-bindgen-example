class Adder {
  int left, right;
public:
  Adder();
  virtual ~Adder();
  void setLeft(int left);
  void setRight(int right);
  int calculate();
};
