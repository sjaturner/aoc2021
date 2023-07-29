load /tmp/second;
toadd=second*0+1;
a=repmat(kron(1:5,toadd),5,1)-1;
second=repmat(second,5,5);
expanded=mod(a+a'+second-1,9)+1;
save expanded expanded
