#include "adder.hpp"

Adder::Adder() {
  left = 0;
  right = 0;
}

Adder::~Adder() {}

void Adder::setLeft(int left) {
  this->left = left;
}

void Adder::setRight(int right) {
  this->right = right;
}

int Adder::calculate() {
  return this->left + this->right;
}
