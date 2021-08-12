#!/bin/bash

awk '/const imports = {}/{flag=1} /if \(typeof input/{flag=0} flag' $1 | sed -e 's/imports.wbg.//' -e '/function/ s/=/:/' -e '/}/ s/;/,/'
